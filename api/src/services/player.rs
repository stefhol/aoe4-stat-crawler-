use crate::player::player_page_server::PlayerPage;
use crate::player::{self, *};
use crate::player::{MatchHistoryEntrie, MatchHistoryReply, RlUserId};
use std::future::Future;
use std::pin::Pin;

use actix::Addr;

use sqlx::PgPool;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{async_trait, Request, Response, Status};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Player {
    pool:PgPool
}

impl Player {
    pub fn new(pool:PgPool) -> Self {
        Self {pool}
    }
}

#[async_trait]
impl PlayerPage for Player {
    async fn get_player_history_matches(
        &self,
        request: Request<RlUserId>,
    ) -> Result<Response<MatchHistoryReply>, Status>{
        Ok(Response::new(MatchHistoryReply{count:3,entry:vec![MatchHistoryEntrie{name:"test".to_string()}]}))
    }
}
