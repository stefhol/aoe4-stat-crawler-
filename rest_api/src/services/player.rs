use std::fmt::{Display, Formatter};
use actix_web::{error, HttpResponse, post, ResponseError, web};
use actix_web::error::JsonPayloadError;
use anyhow::{anyhow, Error};
use itertools::Itertools;
use model::model::request::{MatchType, TeamSize, Versus};
use serde::{Serialize, Deserialize};
use sqlx::PgPool;
use sqlx::types::time::Date;
use crate::db;
use crate::db::RankPageAtTime;
use derive_more::{Display, Error};
use actix_web::http::StatusCode;
use actix_web::web::Data;
use model::model::db::{MatchHistory, MatchHistorySerializable};
use uuid::Uuid;

#[derive(Debug, Display, Error)]
#[display(fmt = "Error {}: {}",name, message)]
struct MyError {
    name:&'static str,
    message: &'static str,
    status_code:StatusCode
}

// Use default implementation for `error_response()` method
impl ResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        self.status_code
    }

    // error on this function implementation
    fn error_response(&self) -> HttpResponse {

        HttpResponse::build(self.status_code).body(self.to_string())
    }
}
#[derive(Clone,Serialize,Deserialize)]
pub struct RlUserId {
    pub rl_user_id: i64,
    pub time: Option<String>,
}
#[derive(Clone, Serialize,Deserialize)]
pub struct MatchHistoryReply {
    pub count: i32,
    pub matches: Vec<MatchHistorySerializable>
}

#[post("/get-player-history")]
async fn get_player_history_matches(
    request: web::Json<RlUserId>,
    pool:Data<PgPool>
) -> actix_web::Result<HttpResponse>{
    // let time = time::Time::;
    let match_history =
        db::get_match_history(pool.as_ref(), request.into_inner().rl_user_id).await;
    if let Ok(match_history) = match_history {

        Ok(HttpResponse::Ok().json(MatchHistoryReply {
            count: match_history.len() as i32,
            matches: match_history.iter().map(|entry|entry.into()).collect_vec(),
        }))
    } else {
        Err(MyError{status_code:StatusCode::BAD_REQUEST,name:"Database",message:"Id is not found in database"}.into())
    }
}
#[derive(Debug, Deserialize, Serialize, Clone)]
struct GetCachedDatesRequest {
    match_type: MatchType,
    versus: Versus,
    team_size: TeamSize,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct GetCachedDatesReply {
    dates: Vec<String>,
}

#[post("/get-cached-dates")]
async fn get_chached_dates(req_body: web::Json<GetCachedDatesRequest>, pool: web::Data<PgPool>) -> actix_web::Result<HttpResponse> {

    //convert
    let match_type = req_body.match_type.to_owned();
    let team_size = req_body.team_size.to_owned();
    let versus = req_body.versus.to_owned();

    let dates = db::get_available_cached_dates(
        pool.get_ref(),
        match_type,
        team_size,
        versus,
    )
        .await;
    match dates {
        Ok(dates) => {
            Ok(HttpResponse::Ok().json(GetCachedDatesReply {
                dates: dates.iter().map(|date| date.format("%F")).collect_vec(),
            }))
        }
        Err(err) =>
            Err(actix_web::error::InternalError::new(err, StatusCode::INTERNAL_SERVER_ERROR).into())
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct CachedRankPageRequest {
    pub player_ids: Vec<i64>,
    pub match_type: MatchType,
    pub versus: Versus,
    pub team_size: TeamSize,
    pub time: Option<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct CachedRankPageReply {
    pub last_leaderboard: Vec<CachedRankPageContent>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct CachedRankPageContent {
    pub rl_user_id: i64,
    pub rank: i32,
    pub elo: i32,
    pub elo_rating: i32,
}

#[post("/get-cached-rank-page")]
async fn get_cached_rank_page(
    request: web::Json<CachedRankPageRequest>,
    pool: web::Data<PgPool>,
) -> actix_web::Result<HttpResponse> {
    fn helper_last_leaderboard(
        last_leaderboard: Result<Vec<RankPageAtTime>, Error>,
    ) -> actix_web::Result<HttpResponse> {
        match last_leaderboard {
            Ok(last_leaderboard) => Ok(HttpResponse::Ok().json(CachedRankPageReply {
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
            Err(err) => Err(actix_web::error::InternalError::new(err, StatusCode::INTERNAL_SERVER_ERROR).into())
        }
    }
    let request: CachedRankPageRequest = request.into_inner();
    if request.player_ids.len() > 100 {
        return Err(MyError{status_code:StatusCode::FORBIDDEN,name:"too many player_ids", message:"Slow down a bit u cannot request more than 100 entries. You think you should? Contact the server admin"}.into());
    }
    if let Some(time) = request.time {
        match Date::parse(&time, "%F") {
            Ok(time) => {
                let last_leaderboard = db::get_rank_page_at_time(
                    &pool,
                    request.player_ids,
                    time,
                    request.match_type.to_owned(),
                    request.team_size.to_owned(),
                    request.versus.to_owned(),
                )
                    .await;
                helper_last_leaderboard(last_leaderboard)
            }
            Err(err) => Err(actix_web::error::ErrorUnprocessableEntity(err))
        }
    } else {
        let last_leaderboard = db::get_latest_rank_page(
            &pool,
            request.player_ids,
            request.match_type.to_owned(),
            request.team_size.to_owned(),
            request.versus.to_owned(),
        )
            .await;
        helper_last_leaderboard(last_leaderboard)
    }
}

