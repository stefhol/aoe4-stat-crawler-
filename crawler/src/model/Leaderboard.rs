use serde::{Deserialize, Serialize};

use super::Request::AgeOfEmpiresLeaderboardRequest;


#[derive(Debug,Clone,Serialize, Deserialize)]
pub struct Leaderboard {
    pub request:Option<AgeOfEmpiresLeaderboardRequest>,
    count:u8,
    items:Vec<LeaderboardEntry>
}
#[derive(Debug,Clone,Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct LeaderboardEntry{
	rlUserId: u64,
	userName: String,
	avatarUrl: String,
	elo: u16,
	eloRating: u16,
	rank: u16,
	region: String,
	wins: u16,
	winPercent: f32,
	losses: u16,
	winStreak: i16,
}
