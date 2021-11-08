use anyhow::Error;
use model::model::db::MatchHistory;
#[allow(unused)]
use model::model::request::{MatchType, Region, TeamSize, Versus};
use sqlx::{ PgPool};
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
    "#,
        rl_user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(match_history)
}
