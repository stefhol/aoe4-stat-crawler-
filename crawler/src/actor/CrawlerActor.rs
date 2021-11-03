use actix::{Actor, AsyncContext, Context, Handler, Message};
use anyhow::Error;
use log::info;

use crate::model::{self, Leaderboard::LeaderboardEntry, Request::{Region, TeamSize, Versus}};

pub struct CrawlerActor {}

impl CrawlerActor {
    pub fn new() {}
}
impl Actor for CrawlerActor {
    type Context = AsyncContext<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        info!("crawler started");
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> actix::Running {
        actix::Running::Stop
    }
}
pub struct AgeRequest(pub u8, pub TeamSize);
impl Message for AgeRequest{
    type Result = Result<(), Error>;
}
impl Handler<AgeRequest> for CrawlerActor {
    type Result = Result<(),Error>;

    fn handle(&mut self, msg: AgeRequest, ctx: &mut Self::Context) -> Self::Result {
        // This will POST a body of `{"lang":"rust","body":"json"}`
        
    }

}
#[derive(Debug)]
pub struct GetUser(pub LeaderboardEntry);

impl actix::Message for GetUser {
    type Result = Result<(), Error>;
}
pub struct DBActor {}
impl Actor for DBActor {
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
