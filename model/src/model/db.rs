use sqlx::types::time::{PrimitiveDateTime};
use uuid::Uuid;
use anyhow;
use super::request::{MatchType, TeamSize, Versus};
use serde::{Serialize, Deserialize};
use time::Date;

#[derive(sqlx::FromRow, Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    pub id: Uuid,
    pub rl_user_id: i64,
    pub username: String,
    pub region: String,
    #[sqlx(default)]
    pub avatar_url: Option<String>,
}

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct MatchHistory {
    pub id: Uuid,
    pub time: PrimitiveDateTime,
    pub elo: i32,
    pub elo_rating: i32,
    pub rank: i32,
    pub wins: i32,
    pub losses: i32,
    pub win_streak: i32,
    pub match_type: MatchType,
    pub team_size: TeamSize,
    pub versus: Versus,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MatchHistorySerializable {
    pub id: Uuid,
    pub time: String,
    pub elo: i32,
    pub elo_rating: i32,
    pub rank: i32,
    pub wins: i32,
    pub losses: i32,
    pub win_streak: i32,
    pub match_type: MatchType,
    pub team_size: TeamSize,
    pub versus: Versus,
}

impl From<MatchHistory> for MatchHistorySerializable {
    fn from(match_history: MatchHistory) -> Self {
        Self {
            id: match_history.id,
            time:match_history.time.format("%F"),
            elo: match_history.elo,
            elo_rating: match_history.elo_rating,
            versus: match_history.versus,
            team_size: match_history.team_size,
            rank: match_history.rank,
            win_streak: match_history.win_streak,
            wins: match_history.wins,
            losses: match_history.losses,
            match_type: match_history.match_type,
        }
    }
}
impl From<&MatchHistory> for MatchHistorySerializable {
    fn from(match_history: &MatchHistory) -> Self {
        Self {
            id: match_history.id,
            time:match_history.time.format("%F"),
            elo: match_history.elo,
            elo_rating: match_history.elo_rating,
            versus: match_history.versus.clone(),
            team_size: match_history.team_size.clone(),
            rank: match_history.rank,
            win_streak: match_history.win_streak,
            wins: match_history.wins,
            losses: match_history.losses,
            match_type: match_history.match_type.clone(),
        }
    }
}
