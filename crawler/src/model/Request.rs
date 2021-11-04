use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
#[derive(Debug,Clone,Serialize, Deserialize)]
pub struct AgeOfEmpiresLeaderboardRequest {
    pub region: Region,
    pub versus: Versus,
    #[serde(rename = "matchType")]
    pub match_type: MatchType,
    #[serde(rename = "teamSize")]
    pub team_size: Option<TeamSize>,
    #[serde(rename = "searchPlayer")]
    pub search_player: String,
    pub page: u32,
    pub count: u32,
}

impl AgeOfEmpiresLeaderboardRequest {
    pub fn new(
        page: u32,
        region: Region,
        team_size: Option<TeamSize>,
        versus: Versus,
    ) -> AgeOfEmpiresLeaderboardRequest {
        AgeOfEmpiresLeaderboardRequest {
            count: 100,
            match_type: MatchType::Unranked,
            page,
            region,
            search_player: "".to_string(),
            team_size: team_size,
            versus,
        }
    }

}
#[derive(Debug,Clone,Serialize, Deserialize,PartialEq)]
#[derive(sqlx::Type)]
#[sqlx(type_name = "match_type")]
pub enum MatchType {
    #[serde(rename = "unranked")]
    #[sqlx(rename = "unranked")]
    Unranked,
    #[serde(rename = "ranked")]
    #[sqlx(rename = "ranked")]
    Ranked,
    #[serde(rename = "custom")]
    #[sqlx(rename = "custom")]
    Custom,
}
#[derive(Debug,Clone,Serialize, Deserialize,PartialEq)]
#[derive(sqlx::Type)]
#[sqlx(type_name = "team_size")]
pub enum TeamSize {
    #[sqlx(rename = "1v1")]
    #[serde(rename = "1v1")]
    T1v1,
    #[sqlx(rename = "2v2")]
    #[serde(rename = "2v2")]
    T2v2,
    #[sqlx(rename = "3v3")]
    #[serde(rename = "3v3")]
    T3v3,
    #[sqlx(rename = "4v4")]
    #[serde(rename = "4v4")]
    T4v4,
    #[sqlx(rename = "custom")]
    #[serde(rename = "custom")]
    Custom,
}
#[derive(Debug,Clone,Serialize_repr, Deserialize_repr,PartialEq)]
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
#[derive(Debug,Clone,Serialize, Deserialize,PartialEq)]
#[derive(sqlx::Type)]
#[sqlx(type_name = "versus")]
pub enum Versus {
    #[serde(rename = "ai")]
    #[sqlx(rename = "ai")]
    AI,
    #[serde(rename = "players")]
    #[sqlx(rename = "players")]
    Players,
}
