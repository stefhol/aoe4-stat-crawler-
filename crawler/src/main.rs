use core::time;
use std::{collections::HashMap, net::SocketAddr, thread};
mod actor;
mod model;
use anyhow::Error;
use clap::{App, Arg};
use log::{error, info};
use reqwest::Client;

use crate::model::{
    Leaderboard::{Leaderboard, LeaderboardEntry},
    Request::{Region, TeamSize, Versus},
};

#[actix_rt::main]
async fn main() -> Result<(), Error> {
    log4rs::init_file("config/log4rs.yml", Default::default()).unwrap();

    let client = reqwest::Client::new();

    let matches = App::new("Age of Empires Leaderboard Crawler")
        .version("1.0")
        .author("Stefan Hoefler")
        .arg(Arg::with_name("DB String")
                .short("db")
                .long("database")
                .value_names(&["Connection String"])
                .help("provides a custom connection string if non provided uses DATABASE_URL out of env. Only supporting PostgresSql")
        )
        .arg(
            Arg::with_name("team_size")
                .value_names(&["team_size"])
                .possible_values(&["1v1", "2v2", "3v3", "4v4"])
                .max_values(1)
                .required(true)
                .short("ts")
                .long("team_size")
                .help("Sets which team size to save")
                .takes_value(true),
        )
        .get_matches();

   let team_size:TeamSize = match matches.value_of("team_size") {
       Some("1v1") => TeamSize::T1v1,
       Some("2v2") => TeamSize::T2v2,
       Some("3v3") => TeamSize::T3v3,
       Some("4v4") => TeamSize::T4v4,
       _=> panic!("Team Size is not correct")
   };

    crawl_aoe4_every_leaderboard(team_size, client).await;
    Ok(())
}

async fn crawl_aoe4_every_leaderboard(team_size: TeamSize, client: Client) {
    //Inital Request has to be performed to know how many sites there are
    let inital_request = crawl_aoe4_singel_leaderboard(1, &team_size, &client).await;
    log_status(&inital_request, &1);
    if let Ok(leaderboard) = inital_request {
        add_leaderboard_page_to_db("", &leaderboard);
        let mut page_number: u32 = 1;
        //get maximum pages
        if let Some(request) = &leaderboard.request {
            let temp_page_number: f32 = leaderboard.count as f32 / request.count as f32;
            page_number = temp_page_number.ceil() as u32
        } else {
            let temp_page_number: f32 = page_number as f32 / 100_f32;
            page_number = temp_page_number.ceil() as u32
        }
        // crawl every page
        for page in 2..page_number + 1 {
            //wait before requesting to not spam the server with too much at the same time
            thread::sleep(time::Duration::from_secs(2));
            let request = crawl_aoe4_singel_leaderboard(page, &team_size, &client).await;
            log_status(&request, &page);
            if let Ok(leaderboard) = request {
                add_leaderboard_page_to_db("", &leaderboard);
            }
        }
    }
}
/**
 * Logs if request was succesfull
 */
fn log_status(leaderboard: &Result<Leaderboard, Error>, page: &u32) {
    match leaderboard {
        Ok(_) => info!("succesfull requested page {}", page),
        Err(err) => {
            error!("Error in leaderboard request at page {}", page);
            error!("{}", err.root_cause().to_string());
        }
    };
}
async fn crawl_aoe4_singel_leaderboard(
    page: u32,
    team_size: &TeamSize,
    client: &Client,
) -> Result<Leaderboard, Error> {
    let mut request = model::Request::AgeOfEmpiresLeaderboardRequest::new(
        page,
        Region::Global,
        Some(team_size.to_owned()),
        Versus::Players,
    );

    let mut res: Leaderboard = client
        .post("https://api.ageofempires.com/api/ageiv/Leaderboard")
        .json(&request)
        .send()
        .await?
        .json()
        .await?;
    //add request to know what type is retrieved
    res.request = Some(request.to_owned());
    Ok(res)
}
fn add_leaderboard_page_to_db(conn: &str, leaderboard: &Leaderboard) {
    info!("Saving Leaderboard Page to db");
}
