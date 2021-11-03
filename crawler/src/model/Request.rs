use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
#[derive(Debug,Clone,Serialize, Deserialize)]
pub struct AgeOfEmpiresLeaderboardRequest {
    region: Region,
    versus: Versus,
    matchType: MatchType,
    teamSize: Option<TeamSize>,
    searchPlayer: String,
    page: u32,
    pub count: u32,
}

impl AgeOfEmpiresLeaderboardRequest {
    pub fn new(
        page: u32,
        region: Region,
        teamSize: Option<TeamSize>,
        versus: Versus,
    ) -> AgeOfEmpiresLeaderboardRequest {
        AgeOfEmpiresLeaderboardRequest {
            count: 100,
            matchType: MatchType::Unranked,
            page,
            region,
            searchPlayer: "".to_string(),
            teamSize,
            versus,
        }
    }

}
#[derive(Debug,Clone,Serialize, Deserialize)]
pub enum MatchType {
    #[serde(rename = "unranked")]
    Unranked,
    #[serde(rename = "ranked")]
    Ranked,
    #[serde(rename = "custom")]
    Custom,
}
#[derive(Debug,Clone,Serialize, Deserialize)]
pub enum TeamSize {
    #[serde(rename = "1v1")]
    T1v1,
    #[serde(rename = "2v2")]
    T2v2,
    #[serde(rename = "3v3")]
    T3v3,
    #[serde(rename = "4v4")]
    T4v4,
}
#[derive(Debug,Clone,Serialize_repr, Deserialize_repr,)]
#[repr(u8)]
pub enum Region {
    Europa = 0,
    MiddleEast = 1,
    Asia = 2,
    NorthAmerica = 3,
    SouthAmerica = 4,
    Oceania = 5,
    Africa = 6,
    Global = 7,
}
#[derive(Debug,Clone,Serialize, Deserialize)]
pub enum Versus {
    #[serde(rename = "ai")]
    AI,
    #[serde(rename = "players")]
    Players,
}
