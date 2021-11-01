use std::{collections::HashMap, net::SocketAddr};
mod model;
use anyhow::Error;
use log::info;

use crate::model::{
    Leaderboard::{Leaderboard, LeaderboardEntry},
    Request::{New, Region, TeamSize, Versus},
};

#[actix_rt::main]
async fn main() -> Result<(), Error> {
    log4rs::init_file("config/log4rs.yml", Default::default()).unwrap();

    let addr: SocketAddr = "[::1]:50051".parse()?;

    info!(
        "Starting server at localip {}:{} ",
        local_ip_address::local_ip().unwrap().to_string(),
        addr.port()
    );
    // This will POST a body of `{"lang":"rust","body":"json"}`
    let mut request = model::Request::AgeOfEmpiresLeaderboardRequest::new(
        1,
        Region::Global,
        Some(TeamSize::T1v1),
        Versus::Players,
    );

    let client = reqwest::Client::new();
    let res: Leaderboard = client
        .post("https://api.ageofempires.com/api/ageiv/Leaderboard")
        .json(&request)
        .send()
        .await?
        // .expect("Cant send")
        .json()
        .await?;
    info!("{:#?}", res);
    Ok(())
}
