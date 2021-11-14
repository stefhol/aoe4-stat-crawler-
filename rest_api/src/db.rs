use anyhow::Error;
use itertools::Itertools;
use model::model::db::MatchHistory;
#[allow(unused)]
use model::model::request::{MatchType, Region, TeamSize, Versus};
use sqlx::{
    types::time::{Date},
    PgPool,
};
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
        limit 100
    "#,
        rl_user_id,
    )
        .fetch_all(pool)
        .await?;

    Ok(match_history)
}
pub struct RankPageAtTime {
    pub rank: i32,
    pub rl_user_id: i64,
    pub elo: i32,
    pub elo_rating: i32,
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
    player_ids: Vec<i64>,
    match_type: MatchType,
    team_size: TeamSize,
    versus: Versus,
) -> Result<Vec<RankPageAtTime>, Error> {
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
        player_ids as Vec<i64>,
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
