use crate::config;
use clap::ArgMatches;
use rocket::fs::NamedFile;
use rocket_dyn_templates::Template;
use std::net;
use std::path::{Path, PathBuf};
use std::str::FromStr;

// root routes
#[get("/")]
async fn index() -> &'static str {
    "Hello,Rocket."
}

#[get("/config")]
async fn config_test() -> Template {
    let cfg = config::load_config();
    Template::render("config", cfg)
}

// static routes
#[get("/<file..>")]
async fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}

// main function of rocket
#[rocket::main]
async fn rocket_main() {
    let cfg = config::load_config();
    println!("{:?}", cfg);
    let teps = Path::new("themes").join(&cfg.site.theme).join("templates");
    if !teps.exists() {
        eprintln!("Theme do not exists or broken.");
        std::process::exit(101);
    }
    let rc_figment = rocket::Config::figment()
        .merge(("template_dir", teps.to_str()))
        .merge(("address", cfg.serve.host))
        .merge(("port", cfg.serve.port));
    rocket::custom(rc_figment)
        .attach(Template::fairing())
        .mount("/", routes![index, config_test])
        .mount("/static", routes![static_files])
        .launch()
        .await
        .unwrap();
}

pub fn serve(s: &ArgMatches) {
    let mut lp_config = config::load_config();
    if let Some(value) = s.value_of("host") {
        if let Ok(host) = net::IpAddr::from_str(value) {
            lp_config = config::Config {
                serve: config::ServeConfig {
                    host: host,
                    ..lp_config.serve
                },
                ..lp_config
            }
            .save(None);
        }
    }
    if let Some(value) = s.value_of("port") {
        if let Ok(port) = u16::from_str(value) {
            config::Config {
                serve: config::ServeConfig {
                    port: port,
                    ..lp_config.serve
                },
                ..lp_config
            }
            .save(None);
        }
    }
    rocket_main();
}
