use serde::{Deserialize, Serialize};

use super::Request::AgeOfEmpiresLeaderboardRequest;


#[derive(Debug,Clone,Serialize, Deserialize)]
pub struct Leaderboard {
    pub request:Option<AgeOfEmpiresLeaderboardRequest>,
    pub count:u32,
    items:Vec<LeaderboardEntry>
}
#[derive(Debug,Clone,Serialize, Deserialize)]
pub struct LeaderboardEntry{
    #[serde(rename = "gameId")]
    game_id: Option<serde_json::Value>,

    #[serde(rename = "userId")]
    user_id: Option<serde_json::Value>,

    #[serde(rename = "rlUserId")]
    rl_user_id: i64,

    #[serde(rename = "userName")]
    user_name: String,

    #[serde(rename = "avatarUrl")]
    avatar_url: Option<String>,

    #[serde(rename = "playerNumber")]
    player_number: Option<serde_json::Value>,

    #[serde(rename = "elo")]
    elo: u64,

    #[serde(rename = "eloRating")]
    elo_rating: u64,

    #[serde(rename = "rank")]
    rank: u64,

    #[serde(rename = "region")]
    region: String,

    #[serde(rename = "wins")]
    wins: u64,

    #[serde(rename = "winPercent")]
    win_percent: f64,

    #[serde(rename = "losses")]
    losses: u64,

    #[serde(rename = "winStreak")]
    win_streak: i64,
}
