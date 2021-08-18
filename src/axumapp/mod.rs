use crate::config;
use crate::logger::info;
use axum::{prelude::*, service::ServiceExt};
use clap::ArgMatches;
use http::StatusCode;
use std::path::PathBuf;
use tower_http::services::{ServeDir, ServeFile};

mod handlers;

#[tokio::main]
pub async fn tokio_run(s: &ArgMatches) {
    let lp_config = config::load_config();

    let app = route("/tags/:tag", get(handlers::tag_pages))
        .route("/:type_/:name", get(handlers::types_pages))
        .route("/:type_", get(handlers::types_index))
        .route("/tags", get(handlers::tags_index))
        .route("/", get(handlers::index))
        .route(
            "/favicon.ico",
            axum::service::get(ServeFile::new("favicon.ico")).handle_error(file_error_handler),
        )
        .nest(
            "/js",
            axum::service::get({
                let lp_config = config::load_config();
                ServeDir::new(
                    &PathBuf::from("themes")
                        .join(&lp_config.site.theme)
                        .join("js"),
                )
                .handle_error(file_error_handler)
            }),
        )
        .nest(
            "/css",
            axum::service::get({
                let lp_config = config::load_config();
                ServeDir::new(
                    &PathBuf::from("themes")
                        .join(&lp_config.site.theme)
                        .join("css"),
                )
                .handle_error(file_error_handler)
            }),
        );

    let mut host: std::net::IpAddr = lp_config.serve.host;
    let mut port: u16 = lp_config.serve.port;
    load_from_str(&mut host, s, "host");
    load_from_str(&mut port, s, "port");
    info(format!("Serving in http://{}:{}", &host, &port));
    axum::Server::bind(&std::net::SocketAddr::new(host, port))
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn file_error_handler(
    error: std::io::Error,
) -> std::result::Result<(StatusCode, String), std::convert::Infallible> {
    Ok::<_, std::convert::Infallible>((
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Unhandled internal error: {}", error),
    ))
}

fn load_from_str<T>(mutdata: &mut T, s: &ArgMatches, keyword: &'static str)
where
    T: std::str::FromStr + Copy,
{
    if let Some(value) = s.value_of(keyword) {
        if let Ok(v) = T::from_str(value) {
            *mutdata = v;
        }
    }
}
