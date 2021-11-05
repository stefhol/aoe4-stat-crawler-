use core::time;
use std::path::Path;
use std::{str::FromStr, thread, time::Duration};

mod actor;
mod model;
use crate::model::request::MatchType;
use crate::model::{
    leaderboard::Leaderboard,
    request::{Region, TeamSize, Versus},
};
use async_recursion::async_recursion;
use anyhow::Error;
use clap::{App, Arg};
use log::{error, info};
use reqwest::{Client, StatusCode};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    query, ConnectOptions, PgPool,
};
use uuid::Uuid;

#[actix_rt::main]
async fn main() -> Result<(), Error> {
    let tries_connection = 0;
    let client = reqwest::Client::builder()
        .user_agent("Leaderboard Crawler for age4.info v1.1")
        .build()?;
    let matches = App::new("Age of Empires Leaderboard Crawler")
        .version("1.0")
        .author("Stefan Hoefler")
        .arg(Arg::with_name("db_string")
                .short("db")
                .long("database")
                .value_names(&["Connection String"])
                .help("provides a custom connection string if non provided uses DATABASE_URL out of env. Only supporting PostgresSql")
        )
        .arg(Arg::with_name("workplace_folder")
                .short("wf")
                .long("workplace_folder")
                .value_names(&["Path to folder where migrations and config is in"])
                .help("For Crawler to work there needs to be a place where migrations and logging conf is located\n
                 Should point to a folder where a config and migrations folder is in\n
                 Can be provided a a env WORKPLACE_FOLDER
                \n")
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

    let workplace_path = match matches.value_of("workplace_folder") {
        Some(val) => val.to_string(),
        _ => dotenv::var("WORKPLACE_FOLDER")
            .expect("Env var WORKPLACE_FOLDER is required or provide one in the command line"),
    };
    let workplace_path = Path::new(&workplace_path);
    let config_location = "config/log4rs.yml";
    let config_location = workplace_path.join(config_location);
    let config_location = config_location.to_str().unwrap();
    let error = format!("config location can not be found: {:?}", &config_location);
    log4rs::init_file(config_location, Default::default()).expect(&error);

    let team_size: TeamSize = match matches.value_of("team_size") {
        Some("1v1") => TeamSize::T1v1,
        Some("2v2") => TeamSize::T2v2,
        Some("3v3") => TeamSize::T3v3,
        Some("4v4") => TeamSize::T4v4,
        _ => panic!("Team Size is not correct"),
    };
    let conn_str = match matches.value_of("db_string") {
        Some(val) => val.to_string(),
        _ => dotenv::var("DATABASE_URL")
            .expect("Env var DATABASE_URL is required or provide one in the command line"),
    };

    let pool = PgPoolOptions::new()
        .connect_with(
            PgConnectOptions::from_str(&conn_str)
                .unwrap()
                .application_name("Age4.info Crawler")
                .log_statements(log::LevelFilter::Trace)
                .log_slow_statements(log::LevelFilter::Trace, Duration::from_secs(1))
                .to_owned(),
        )
        .await?;
    //start db migration check
    match sqlx::migrate!().run(&pool).await {
        Ok(_) => info!("Applying migrations"),
        Err(_) => error!("Migrations folder not found skipping"),
    };
    crawl_aoe4_every_leaderboard(team_size, client, &pool, tries_connection).await;
    Ok(())
}
#[async_recursion]
async fn prepare_db_inject(
    page: u32,
    transaction: &PgPool,
    team_size: &TeamSize,
    client: &Client,
    mut tries_connection: u8,
) {
    //wait before requesting to not spam the server with too much at the same time
    thread::sleep(time::Duration::from_secs(2));
    let request = crawl_aoe4_singel_leaderboard(page, &team_size, &client).await;
    log_status(&request, &page);
    if let Ok(leaderboard) = request {
        add_leaderboard_page_to_db(transaction, &leaderboard).await;
    } else if let Err(err) = request {
        error!("{}", err);
        if tries_connection < 10 {
            error!("Retrying one more time");
            tries_connection += 1;
            prepare_db_inject(page, transaction, team_size, client, tries_connection).await;
        } else {
            error!(
                "Server failed a total of {} times. Aborting",
                tries_connection
            );
        }
    }
}
//////
/// Gets every page out of the leaderboard then passes this page to crawl_aoe4_single_leaderboard_page
#[async_recursion]
async fn crawl_aoe4_every_leaderboard(
    team_size: TeamSize,
    client: Client,
    transaction: &PgPool,
    mut tries_connection: u8,
) {
    //Inital Request has to be performed to know how many sites there are
    tries_connection += 1;
    let inital_request = crawl_aoe4_singel_leaderboard(1, &team_size, &client).await;
    log_status(&inital_request, &1);
    if let Ok(leaderboard) = inital_request {
        add_leaderboard_page_to_db(transaction, &leaderboard).await;
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
            prepare_db_inject(page, transaction, &team_size, &client, tries_connection).await;
        }
    } else if let Err(err) = inital_request {
        error!("{}", err);

        if tries_connection < 5 {
            thread::sleep(time::Duration::from_secs(2));
            error!(
                "Error trying to get initial page. Retry: {}",
                &tries_connection
            );
            crawl_aoe4_every_leaderboard(team_size, client, &transaction, tries_connection).await;
        } else {
            error!(
                "Cant get connection to server after {} tries",
                &tries_connection
            );
        }
    }
}
///
/// Prints if it was succesfull to request the aoe4 db api
fn log_status(leaderboard: &Result<Leaderboard, Error>, page: &u32) {
    match leaderboard {
        Ok(_) => info!("succesfull requested page {}", page),
        Err(err) => {
            error!("Error in leaderboard request at page {}", page);
            error!("{}", err.root_cause().to_string());
        }
    };
}
//////
/// Crawls single page of a leaderboard
async fn crawl_aoe4_singel_leaderboard(
    page: u32,
    team_size: &TeamSize,
    client: &Client,
) -> Result<Leaderboard, Error> {
    let request = model::request::AgeOfEmpiresLeaderboardRequest::new(
        page,
        Region::Global,
        Some(team_size.to_owned()),
        Versus::Players,
    );

    let res = client
        .post("https://api.ageofempires.com/api/ageiv/Leaderboard")
        .json(&request)
        .send()
        .await;
    let res: Result<reqwest::Response, anyhow::Error> = match res {
        Ok(response) => {
            if response.status() == StatusCode::OK {
                Ok(response)
            } else {
                Err(anyhow::anyhow!("{} Status Code", response.status()))
            }
        }
        Err(err) => Err(anyhow::anyhow!("{:?}", err)),
    };
    let res = res?;
    let res_str = res.text().await?;
    let res_json:Result<Leaderboard,_> = serde_json::from_str(&res_str);
    return match res_json {
        Ok(mut leaderboard) => {
            leaderboard.request = Some(request);
            Ok(leaderboard)
        },
        Err(err) => {
            error!("Error Parsing content as Leaderboard.");
            error!("Content: {}", res_str);
            Err(anyhow::anyhow!("{}", err))
        }
    };
}
//////
/// Adds Leaderboard page to the DB
async fn add_leaderboard_page_to_db(pool: &PgPool, leaderboard: &Leaderboard) {
    info!("Saving Leaderboard Page to db");
    if let Some(request) = &leaderboard.request {
        for leaderboard_entry in &leaderboard.items {
            //check if player exists
            let player_result = query!(
                "SELECT id FROM player WHERE rl_user_id = $1",
                leaderboard_entry.rl_user_id
            )
            .fetch_optional(pool)
            .await;
            //db player is already in
            if let Ok(player_option) = player_result {
                let mut player_id: Uuid = Uuid::nil();
                //db player option
                if let Some(db_player) = player_option {
                    let rows_affected = query!(
                        r#"
                    UPDATE player
                    SET username = $2, avatar_url = $3, region = $4
                    WHERE id = $1
                    "#,
                        db_player.id,
                        leaderboard_entry.username,
                        leaderboard_entry.avatar_url,
                        leaderboard_entry.region
                    )
                    .execute(pool)
                    .await;
                    //saving db player id
                    player_id = db_player.id;
                    if let Ok(_) = rows_affected {
                        info!(target:"db", "update player");
                    } else {
                        error!(target:"db","updating player error")
                    }
                }
                //db player needs to be inserted
                else {
                    let query_player = query!(
                        r#"
                    INSERT INTO player ( rl_user_id, username,region,avatar_url) 
                    VALUES ( $1 ,$2,$3,$4 ) RETURNING id"#,
                        leaderboard_entry.rl_user_id,
                        leaderboard_entry.username,
                        leaderboard_entry.region,
                        leaderboard_entry.avatar_url
                    )
                    .fetch_one(pool)
                    .await;
                    if let Ok(player) = query_player {
                        info!(target:"db", "insert player");
                        player_id = player.id;
                    } else {
                        error!(target:"db","error inserting player");
                    }
                }
                //lets check if the player played a new match
                let query_match_history = query!(
                    r#"
                    SELECT 
                    match_type as "match_type:MatchType",
                    team_size as "team_size:TeamSize",
                    versus as "versus:Versus",
                    rank,
                    elo,
                    wins,
                    losses,
                    win_streak
                     FROM match_history 
                    join player_match_history on match_history.id = match_history_id
                    where player_id = $1
                    ORDER BY time DESC
                "#,
                    player_id
                )
                .fetch_optional(pool)
                .await;
                match query_match_history {
                    Ok(_) => info!(target:"db","selecting newest match from player:{}",player_id),
                    _ => {
                        error!(target:"db","error selecting newest match from player:{}",player_id)
                    }
                }
                let mut same_match = false;
                if let Ok(possible_latest_match) = query_match_history {
                    if let Some(latest_match) = possible_latest_match {
                        let request_team_size = match &request.team_size {
                            Some(team_size) => team_size.to_owned(),
                            None => TeamSize::Custom,
                        };
                        if latest_match.match_type == request.match_type {
                            if latest_match.team_size == request_team_size {
                                if latest_match.versus == request.versus {
                                    if latest_match.rank == leaderboard_entry.rank {
                                        if latest_match.elo == leaderboard_entry.elo {
                                            if latest_match.wins == leaderboard_entry.wins {
                                                if latest_match.losses == leaderboard_entry.losses {
                                                    if latest_match.win_streak
                                                        == leaderboard_entry.win_streak
                                                    {
                                                        same_match = true;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                if same_match == false {
                    //now we have the player
                    //and he played a new match
                    //add the match and connect it
                    let team_size = match &leaderboard.request {
                        Some(request) => match request.team_size.to_owned() {
                            Some(team_size) => team_size,
                            None => TeamSize::Custom,
                        },
                        None => TeamSize::Custom,
                    };
                    let match_type = match &leaderboard.request {
                        Some(request) => request.match_type.to_owned(),
                        None => MatchType::Custom,
                    };
                    let versus = match &leaderboard.request {
                        Some(request) => request.versus.to_owned(),
                        None => Versus::Players,
                    };
                    // Insert Match history
                    let query_match_history = query!(
                        r#"
                    INSERT INTO match_history(
                        elo,
                        elo_rating,
                        rank,
                        wins,
                        losses,
                        win_streak,
                        match_type,
                        team_size,
                        versus
                    )
                    VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9) RETURNING id
                "#,
                        leaderboard_entry.elo,
                        leaderboard_entry.elo_rating,
                        leaderboard_entry.rank,
                        leaderboard_entry.wins,
                        leaderboard_entry.losses,
                        leaderboard_entry.win_streak,
                        match_type as MatchType,
                        team_size as TeamSize,
                        versus as Versus
                    )
                    .fetch_one(pool)
                    .await;
                    match query_match_history {
                        Ok(_) => info!(target:"db","insert match_history"),
                        _ => error!(target:"db","error inserting match_history"),
                    };

                    // Connect match history with player
                    if let Ok(match_history) = query_match_history {
                        let combine = query!(
                            r#"
                        INSERT INTO player_match_history(player_id,match_history_id)
                        VALUES($1,$2)
                        "#,
                            player_id,
                            match_history.id
                        )
                        .execute(pool)
                        .await;
                        match combine {
                            Ok(_) => info!(target:"db","insert player_match_history"),
                            _ => error!(target:"db","error inserting player_match_history"),
                        };
                    }
                } else {
                    info!(target:"db","Skipping insert match for user: {}. Last match is identical to the new match",player_id);
                }
            } else {
                error!(target:"db",
                    "select player error! rl_user_id = {}",
                    leaderboard_entry.rl_user_id
                )
            }
        }
    } else {
        error!("Leaderboard has no request")
    }
}
