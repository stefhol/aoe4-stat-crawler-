use serde::{Deserialize, Serialize};

use super::request::AgeOfEmpiresLeaderboardRequest;


#[derive(Debug,Clone,Serialize, Deserialize)]
pub struct Leaderboard {
    pub request:Option<AgeOfEmpiresLeaderboardRequest>,
    pub count:u32,
    pub items:Vec<LeaderboardEntry>
}
#[derive(Debug,Clone,Serialize, Deserialize)]
pub struct LeaderboardEntry{
    #[serde(rename = "gameId")]
    pub game_id: Option<serde_json::Value>,

    #[serde(rename = "userId")]
    pub user_id: Option<serde_json::Value>,

    #[serde(rename = "rlUserId")]
    pub rl_user_id: i64,

    #[serde(rename = "userName")]
    pub username: String,

    #[serde(rename = "avatarUrl")]
    pub avatar_url: Option<String>,

    #[serde(rename = "playerNumber")]
    pub player_number: Option<serde_json::Value>,

    #[serde(rename = "elo")]
    pub elo: i32,

    #[serde(rename = "eloRating")]
    pub elo_rating: i32,

    #[serde(rename = "rank")]
    pub rank: i32,

    #[serde(rename = "region")]
    pub region: String,

    #[serde(rename = "wins")]
    pub wins: i32,

    #[serde(rename = "winPercent")]
    pub win_percent: f64,

    #[serde(rename = "losses")]
    pub losses: i32,

    #[serde(rename = "winStreak")]
    pub win_streak: i32,
}
