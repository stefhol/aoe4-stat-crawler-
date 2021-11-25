use anyhow::{ Error};
use itertools::Itertools;
use model::model::db::{MatchHistory, Player};
#[allow(unused)]
use model::model::request::{MatchType, Region, TeamSize, Versus};
use redis::aio::MultiplexedConnection;
use redis::{AsyncCommands, RedisWrite, ToRedisArgs, Value};
use serde::{Deserialize, Serialize};
use sqlx::{types::time::Date, PgPool};

pub async fn get_player(pool: &PgPool, rl_user_id: i64) -> Result<Player, Error> {
    let player = sqlx::query_as!(
        Player,
        "select * from player where rl_user_id = $1",
        rl_user_id
    )
        .fetch_one(pool)
        .await?;
    Ok(player)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SearchPlayer {
    pub username: String,
    pub rl_user_id: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SearchPlayers {
    pub players: Vec<SearchPlayer>,
    pub count: u32,
}

pub async fn search_player(
    pool: &PgPool,
    name: &str,
) -> Result<SearchPlayers, Error> {
    let players = sqlx::query_as!(
        SearchPlayer,
       "SELECT rl_user_id, username FROM player WHERE  LOWER(username) like LOWER($1) LIMIT 10",
        format!("%{}%",name)
    )
        .fetch_all(pool)
        .await?;
    Ok(SearchPlayers { count: *&players.len() as u32, players })
}

///get all matches from specific player
pub async fn get_match_history(pool: &PgPool, rl_user_id: i64) -> Result<Vec<MatchHistory>, Error> {
    let match_history = sqlx::query_as!(
        MatchHistory,
        r#"
        select
                match_type as "match_type:MatchType",
                    team_size as "team_size:TeamSize",
                    versus as "versus:Versus",
                rank,
                elo_rating,
                match_history.id,
                time as "time!",
                elo,
                wins,
                losses,
                win_streak
                from player
        INNER join player_match_history on player.id = player_id
        inner join match_history on match_history.id = match_history_id
        where player.rl_user_id = $1
        order by time DESC
        limit 100
    "#,
        rl_user_id,
    )
        .fetch_all(pool)
        .await?;

    Ok(match_history)
}

#[derive(Deserialize, Serialize)]
pub struct RankPageAtTime {
    pub rank: i32,
    pub rl_user_id: i64,
    pub elo: i32,
    pub elo_rating: i32,
}


impl ToRedisArgs for &RankPageAtTime {
    fn write_redis_args<W>(&self, out: &mut W)
        where
            W: ?Sized + RedisWrite,
    {
        out.write_arg_fmt(serde_json::to_string(self).expect("Can't serialize RankPageAtTime as string"))
    }
}


///Gets match entries from players at specific date
pub async fn get_rank_page_at_time(
    pool: &PgPool,
    player_ids: Vec<i64>,
    time: Date,
    match_type: MatchType,
    team_size: TeamSize,
    versus: Versus,
) -> Result<Vec<RankPageAtTime>, Error> {
    let match_history = sqlx::query!(
        r#"
        select

                rank,
                player_subset.rl_user_id,
                elo,
                elo_rating
                from (
                    SELECT * from player
                    where player.rl_user_id = any($1)
                )  as player_subset

        INNER join player_match_history on player_subset.id = player_id
        inner join (
            SELECT time,match_type,team_size,versus,elo,elo_rating,rank,id FROM match_history
            WHERE date(time) = date($2)
            AND
            match_type = $3
            AND
            team_size = $4
            AND
            versus = $5
        ) as match_history_subset
         on match_history_subset.id = match_history_id
    "#,
        player_ids as Vec<i64>,
        time as Date,
        match_type as MatchType,
        team_size as TeamSize,
        versus as Versus
    )
        .fetch_all(pool)
        .await?;
    Ok(match_history
        .iter()
        .map(|record| RankPageAtTime {
            elo: record.elo,
            elo_rating: record.elo_rating,
            rank: record.rank,
            rl_user_id: record.rl_user_id,
        })
        .collect_vec())
}

///Gets the last match entry from each player
pub async fn get_latest_rank_page(
    pool: &PgPool,
    redis_conn: &MultiplexedConnection,
    player_ids: Vec<i64>,
    match_type: MatchType,
    team_size: TeamSize,
    versus: Versus,
) -> Result<Vec<RankPageAtTime>, Error> {
    let mut redis_conn = redis_conn.clone();
    let mut cached_rank_page: Vec<RankPageAtTime> = vec![];
    let mut player_ids_to_query = vec![];
    //generates a redis key for cached-leaderboards
    let get_redis_key = |player_id: i64| format!("cached-leaderboard-{}", player_id);
    //go through each player_id to find a value in the redis cache
    for player_id in player_ids {
        let redis_key = get_redis_key(player_id);
        let redis_result = redis_conn.get(&redis_key).await.unwrap();
        match redis_result {
            Value::Nil => {
                player_ids_to_query.push(player_id);
            }
            Value::Data(val) => {
                cached_rank_page.push(serde_json::from_slice(&val)?);
            }
            _ => {}
        };
    }
    if player_ids_to_query.len() > 0 {
        //if player_ids are not all cached get the rest from the db
        let match_history = sqlx::query!(
        r#"
        select

                distinct on (player_subset.rl_user_id) rl_user_id,
                rank,
                elo,
                elo_rating
                from (
                    SELECT * from player
                    where player.rl_user_id = any($1)
                )  as player_subset

        INNER join player_match_history on player_subset.id = player_id
        inner join (
            SELECT time,match_type,team_size,versus,elo,elo_rating,rank,id FROM match_history
            WHERE
            match_type = $2
            AND
            team_size = $3
            AND
            versus = $4
            ORDER BY time DESC
        ) as match_history_subset
         on match_history_subset.id = match_history_id
        "#,
            player_ids_to_query as Vec<i64>,
            match_type as MatchType,
            team_size as TeamSize,
            versus as Versus
        )
            .fetch_all(pool)
            .await?;
        //convert match_history to known rust struct
        let mut match_history: Vec<RankPageAtTime> = match_history
            .iter()
            .map(|record| RankPageAtTime {
                elo: record.elo,
                elo_rating: record.elo_rating,
                rank: record.rank,
                rl_user_id: record.rl_user_id,
            })
            .collect_vec();
        //insert missing match_history in  cache
        for rank_page in &match_history {
            let redis_key = get_redis_key(rank_page.rl_user_id);
            redis::pipe()
                .atomic()
                .set(&redis_key, &rank_page)
                .expire(&redis_key, 600)
                .query_async(&mut redis_conn).await?;
        };
        //add db match_history to cached_rank_page
        cached_rank_page.append(&mut match_history);
    }
    Ok(cached_rank_page)
}

pub async fn get_available_cached_dates(
    pool: &PgPool,
    match_type: MatchType,
    team_size: TeamSize,
    versus: Versus,
) -> Result<Vec<Date>, Error> {
    let dates = sqlx::query!(
        r#"
            SELECT DISTINCT date(time) FROM match_history
            WHERE match_type = $1
            AND
            team_size = $2
            AND
            versus = $3
            AND
            time is not NULL
            LIMIT 365
    "#,
        match_type as MatchType,
        team_size as TeamSize,
        versus as Versus
    )
        .fetch_all(pool)
        .await?;
    Ok(dates.iter().map(|date| date.date.unwrap()).collect_vec())
}
