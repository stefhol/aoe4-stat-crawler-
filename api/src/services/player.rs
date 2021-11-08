use crate::db;
use crate::player::player_page_server::PlayerPage;
use crate::player::{MatchHistoryEntry, MatchHistoryReply, RlUserId};
use itertools::Itertools;

use model::model::db::MatchHistory;
use serde_json;
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
        let match_history =
            db::get_match_history(&self.pool, request.into_inner().rl_user_id).await;
        if let Ok(match_history) = match_history {
            let match_history: Vec<MatchHistoryEntry> = match_history
                .iter()
                .map(|my_match| my_match.from_match_history())
                .collect_vec();
            Ok(Response::new(MatchHistoryReply {
                count: match_history.len() as i32,
                entry: match_history,
            }))
        } else {
            Err(Status::invalid_argument("Id not found"))
        }
    }
}
