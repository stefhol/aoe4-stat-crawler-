use actix::{Actor, Context, Handler};
use anyhow::Error;
use log::info;

use crate::model::Leaderboard::LeaderboardEntry;

pub struct CrawlerActor{

}

impl CrawlerActor {
    pub fn new(){}
}
impl Actor for CrawlerActor {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        info!("crawler started");
    }
    
    fn stopping(&mut self, ctx: &mut Self::Context) -> actix::Running {
        actix::Running::Stop
    }
}
#[derive(Debug)]
pub struct GetUser(pub LeaderboardEntry);

impl actix::Message for GetUser {
    type Result = Result<(), Error>;
}
pub struct DBActor {
}
impl Actor for DBActor{
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        info!(target: "db","db actor started");
    }

}
impl Handler<GetUser> for DBActor {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: GetUser, ctx: &mut Self::Context) -> Self::Result {
        //do something
        Ok(())
    }
}