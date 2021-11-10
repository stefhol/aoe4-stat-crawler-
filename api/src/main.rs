use anyhow::Error;
use sqlx::{
    self,
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions,
};
use std::{env, net::SocketAddr, str::FromStr, time::Duration};
mod services;
use tonic;
mod proto_build;
use proto_build::player;
mod db;
use log::info;

use tonic::transport::Server;

#[actix_rt::main]
async fn main() -> Result<(), Error> {
    match log4rs::init_file("./config/log4rs.yml", Default::default()) {
        Ok(_) => (),
        Err(_) => (),
    };
    
    let port = dotenv::var("PORT").expect("no PORT in env");
    let addr: SocketAddr = format!("[::1]:{}", port).parse()?;
    let conn_str = dotenv::var("DATABASE_URL").expect("no DATABASE_URL in env");

    let pool = PgPoolOptions::new()
        .connect_with(
            PgConnectOptions::from_str(&conn_str)
                .unwrap()
                .application_name("Age4 API Service")
                .log_statements(log::LevelFilter::Trace)
                .log_slow_statements(log::LevelFilter::Trace, Duration::from_secs(1))
                .to_owned(),
        )
        .await?;

    info!(
        "Starting server at localip http://{}:{} ",
        local_ip_address::local_ip().unwrap().to_string(),
        addr.port()
    );
    Server::builder()
        .add_service(player::player_page_server::PlayerPageServer::new(
            services::player::Player::new(pool),
        ))
        .serve(addr)
        .await?;

    Ok(())
}
