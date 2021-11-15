use std::net::{SocketAddr,Ipv4Addr,IpAddr};
use std::str::FromStr;
use std::time::Duration;
use actix_cors::Cors;
use actix_web::{ App, HttpServer,middleware, http};
use actix_web::web::Data;
use log::info;
use sqlx::ConnectOptions;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
mod services;
mod db;

use crate::services::player::{get_cached_rank_page, get_chached_dates, get_player_history_matches};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match log4rs::init_file("./config/log4rs.yml", Default::default()) {
        Ok(_) => (),
        Err(_) => (),
    };

    let port = dotenv::var("PORT").expect("no PORT in env");
    let addr: SocketAddr = format!("127.0.0.1:{}", port).parse().expect("Port is in wrong format");
    let conn_str = dotenv::var("DATABASE_URL").expect("no DATABASE_URL in env");

    let pool = PgPoolOptions::new()
        .connect_with(
            PgConnectOptions::from_str(&conn_str)
                .unwrap()
                .application_name("Age4 REST API Service")
                .log_statements(log::LevelFilter::Trace)
                .log_slow_statements(log::LevelFilter::Trace, Duration::from_secs(1))
                .to_owned(),
        )
        .await
        .expect("can not connect to db");

    info!(
        "Starting server at localip http://{}:{} ",
        local_ip_address::local_ip()
            .unwrap_or(IpAddr::V4(Ipv4Addr::UNSPECIFIED))
            .to_string(),
        addr.port()
    );
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:3000")
            .allowed_origin("http://localhost:3000")
            .allowed_origin("https://age4.info")
            .allowed_origin("http://age4.info")
            .allowed_origin("https://www.age4.info")
            .allowed_origin("http://www.age4.info")
            .allowed_methods(vec!["POST"])
            .allowed_header(http::header::CONTENT_TYPE)
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS)
            .allowed_header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN)
            .allowed_header(http::header::ACCEPT)
            .max_age(3600);
        App::new()
            .app_data(Data::new(pool.clone()))
            .wrap(middleware::Compress::default())
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .service(get_chached_dates)
            .service(get_cached_rank_page)
            .service(get_player_history_matches)
    })
    .bind(addr)?.workers(1)
    .run()
    .await
}
