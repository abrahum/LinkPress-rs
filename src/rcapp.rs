use crate::config;
use crate::utils;
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

#[get("/favicon.ico")]
async fn favicon() -> Option<NamedFile> {
    NamedFile::open(Path::new("favicon.ico")).await.ok()
}

// #[get("/config")]
// async fn config_test() -> Template {
//     let cfg = config::load_config();
//     Template::render("config", cfg)
// }

// static routes
#[get("/<file..>")]
async fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}

// types routes
#[get("/<type_>/<name>", rank = 2)]
async fn types_page(type_: &str, name: &str) -> Template {
    let mut template_name = type_;
    let md_string = utils::get_page(&type_, name).unwrap();
    let context = utils::build_pagedata(name, md_string);
    if let Some(t) = &context.front_matter.template {
        template_name = &t;
    }
    Template::render(String::from(template_name), context)
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
        .mount("/", routes![index, types_page, favicon])
        .mount("/static", routes![static_files])
        .launch()
        .await
        .unwrap();
}

pub fn serve(s: &ArgMatches) {
    let mut lp_config = config::load_config();
    if let Some(value) = s.value_of("host") {
        if let Ok(host) = net::IpAddr::from_str(value) {
            lp_config.serve.host = host;
            lp_config = lp_config.save(None);
        }
    }
    if let Some(value) = s.value_of("port") {
        if let Ok(port) = u16::from_str(value) {
            lp_config.serve.port = port;
            lp_config.save(None);
        }
    }
    rocket_main();
}
