use crate::db;
use crate::proto_build::player::player_page_server::PlayerPage;
use crate::proto_build::player::{
    CachedRankPageReply, CachedRankPageRequest, DateReply, GetCachedDatesRequest,
    MatchHistoryEntry, MatchHistoryReply, RlUserId,
};
use anyhow::Error;
use itertools::Itertools;

use crate::db::RankPageAtTime;
use crate::proto_build::player::cached_rank_page_reply::CachedRankPageContent;
use model::model::db::MatchHistory;
use model::model::request::{MatchType, TeamSize, Versus};
use serde_json;
use serde_json::json;
use sqlx::types::time::Date;
use sqlx::PgPool;
use tonic::{async_trait, Request, Response, Status};
#[derive(Clone, Debug)]
pub struct Player {
    pool: PgPool,
}
trait FromMatchHistory {
    fn from_match_history(&self) -> MatchHistoryEntry;
}
impl FromMatchHistory for MatchHistory {
    fn from_match_history(&self) -> MatchHistoryEntry {
        MatchHistoryEntry {
            id: self.id.to_string(),
            match_type: serde_json::to_value(&self.match_type)
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
            team_size: serde_json::to_value(&self.team_size)
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
            elo: self.elo,
            time: self.time.format("%F"),
            rank: self.rank,
            elo_rating: self.elo_rating,
            losses: self.losses,
            wins: self.wins,
            win_streak: self.win_streak,
            versus: serde_json::to_value(&self.versus)
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
        }
    }
}
impl Player {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PlayerPage for Player {
    async fn get_player_history_matches(
        &self,
        request: Request<RlUserId>,
    ) -> Result<Response<MatchHistoryReply>, Status> {
        // let time = time::Time::;
        let match_history =
            db::get_match_history(&self.pool, request.into_inner().rl_user_id).await;
        if let Ok(match_history) = match_history {
            let match_history: Vec<MatchHistoryEntry> = match_history
                .iter()
                .map(|my_match| my_match.from_match_history())
                .collect_vec();
            Ok(Response::new(MatchHistoryReply {
                count: match_history.len() as i32,
                matches: match_history,
            }))
        } else {
            Err(Status::invalid_argument("Id not found"))
        }
    }

    async fn get_cached_dates(
        &self,
        request: Request<GetCachedDatesRequest>,
    ) -> Result<Response<DateReply>, Status> {
        //convert
        let request = request.into_inner();
        let converted_values =
            MVTConverter::convert(&request.match_type, &request.versus, &request.team_size);
        match converted_values {
            Ok(converted_values) => {
                let dates = db::get_available_cached_dates(
                    &self.pool,
                    converted_values.match_type,
                    converted_values.team_size,
                    converted_values.versus,
                )
                .await;
                if let Ok(dates) = dates {
                    Ok(Response::new(DateReply {
                        dates: dates.iter().map(|date| date.format("%F")).collect_vec(),
                    }))
                } else {
                    Err(Status::unknown("Error Code: 452131"))
                }
            }
            Err(err) => return Err(Status::invalid_argument(err.to_string())),
        }
    }

    async fn get_cached_rank_page(
        &self,
        request: Request<CachedRankPageRequest>,
    ) -> Result<Response<CachedRankPageReply>, Status> {
        fn helper_last_leaderboard(
            last_leaderboard: Result<Vec<RankPageAtTime>, Error>,
        ) -> Result<Response<CachedRankPageReply>, Status> {
            match last_leaderboard {
                Ok(last_leaderboard) => Ok(Response::new(CachedRankPageReply {
                    last_leaderboard: last_leaderboard
                        .iter()
                        .map(|leaderboard_entry| CachedRankPageContent {
                            rank: leaderboard_entry.rank,
                            rl_user_id: leaderboard_entry.rl_user_id,
                            elo_rating: leaderboard_entry.elo_rating,
                            elo: leaderboard_entry.elo,
                        })
                        .collect_vec(),
                })),
                Err(err) => Err(Status::invalid_argument(err.to_string())),
            }
        }
        let request: CachedRankPageRequest = request.into_inner();
        match MVTConverter::convert(&request.match_type, &request.versus, &request.team_size) {
            Ok(converted_values) => {
                if let Some(time) = request.time {
                    match Date::parse(time, "%F") {
                        Ok(time) => {
                            let last_leaderboard = db::get_rank_page_at_time(
                                &self.pool,
                                request.player_ids,
                                time,
                                converted_values.match_type,
                                converted_values.team_size,
                                converted_values.versus,
                            )
                            .await;
                            helper_last_leaderboard(last_leaderboard)
                        }
                        Err(err) => Err(Status::invalid_argument(err.to_string())),
                    }
                } else {
                    let last_leaderboard = db::get_latest_rank_page(
                        &self.pool,
                        request.player_ids,
                        converted_values.match_type,
                        converted_values.team_size,
                        converted_values.versus,
                    )
                    .await;
                    helper_last_leaderboard(last_leaderboard)
                }
            }
            Err(err) => Err(Status::invalid_argument(err.to_string())),
        }
    }
}

struct MVTConverter {
    pub match_type: MatchType,
    pub versus: Versus,
    pub team_size: TeamSize,
}
impl MVTConverter {
    fn convert(match_type: &str, versus: &str, team_size: &str) -> Result<Self, Error> {
        //convert
        let match_type: MatchType = serde_json::from_value(json!(match_type))?;
        let versus: Versus = serde_json::from_value(json!(versus))?;
        let team_size: TeamSize = serde_json::from_value(json!(team_size))?;
        Ok(Self {
            match_type,
            versus,
            team_size,
        })
    }
}
