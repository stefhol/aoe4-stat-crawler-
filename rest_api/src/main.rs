use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{http, middleware, App, HttpServer};
use log::info;
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{ConnectOptions};
use std::fs::read_dir;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::Path;
use std::str::FromStr;
use std::time::Duration;

mod db;
mod services;
use crate::services::player::{
    get_cached_rank_page, get_chached_dates, get_player_history_matches, search_player
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match log4rs::init_file("./config/log4rs.yml", Default::default()) {
        Ok(_) => (),
        Err(_) => (),
    };
    let builder: Option<SslAcceptorBuilder> = match dotenv::var("SSL") {
        Ok(val) => {
            let mut local_builder = SslAcceptor::mozilla_modern_v5(SslMethod::tls()).unwrap();
            for path in read_dir(val.clone())
                .expect(&format!("Cannot read cert dir -> {}", val.clone()))
                .map(|dir| {
                    dir.expect(&format!("cannot read possible cert in {}", val.clone()))
                        .path()
                })
            {
                let path = Path::new(&path);
                if path.ends_with("key.pem") {
                    local_builder
                        .set_private_key_file(path, SslFiletype::PEM)
                        .unwrap();
                } else if path.ends_with("cert.pem") {
                    local_builder.set_certificate_chain_file(path).unwrap();
                } else {
                    info!("found in folder {}", path.to_str().unwrap());
                }
            }
            Some(local_builder)
        }
        Err(_) => None,
    };
    let port = dotenv::var("PORT").expect("no PORT in env");
    let address = dotenv::var("ADDRESS");
    let addr: SocketAddr = match address {
        Ok(val) => format!("{}:{}", val.clone(), port.clone())
            .parse()
            .expect(&format!(
                "cant parse socketAddress, {}:{}",
                val.clone(),
                port.clone()
            )),
        Err(_) => format!("0.0.0.0:{}", port)
            .parse()
            .expect("Port is in wrong format"),
    };
    let conn_str_db = dotenv::var("DATABASE_URL").expect("no DATABASE_URL in env");
    let pool = PgPoolOptions::new()
        .connect_with(
            PgConnectOptions::from_str(&conn_str_db)
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
    info!("Binding to {}", &addr.to_string());
    fn service_config(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(get_chached_dates)
            .service(get_cached_rank_page)
            .service(get_player_history_matches)
            .service(search_player);
    }
    match builder {
        Some(builder) => {
            HttpServer::new(move || {
                info!("running in ssl mode");
                let cors = Cors::default()
                    .allowed_origin("http://127.0.0.1:3000")
                    .allowed_origin("http://localhost:3000")
                    .allowed_origin("https://age4.info")
                    .allowed_origin("http://age4.info")
                    .allowed_origin("https://www.age4.info")
                    .allowed_origin("http://www.age4.info")
                    .allow_any_method()
                    .allow_any_header()
                    .allow_any_origin()
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
                    .configure(service_config)
            })
            .bind_openssl(addr, builder)?
            .workers(1)
            .run()
            .await
        }
        None => {
            HttpServer::new(move || {
                info!("running in non ssl mode");
                let cors = Cors::default()
                    .allowed_origin("http://127.0.0.1:3000")
                    .allowed_origin("http://localhost:3000")
                    .allowed_origin("https://age4.info")
                    .allowed_origin("http://age4.info")
                    .allowed_origin("https://www.age4.info")
                    .allowed_origin("http://www.age4.info")
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600);
                App::new()
                    .app_data(Data::new(pool.clone()))
                    .wrap(middleware::Compress::default())
                    .wrap(cors)
                    .wrap(middleware::Logger::default())
                    .configure(service_config)
            })
            .bind(addr)?
            .workers(1)
            .run()
            .await
        }
    }
}

