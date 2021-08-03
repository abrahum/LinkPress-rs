use crate::config;
use rocket::fs::NamedFile;
use rocket_dyn_templates::Template;
use std::path::{Path, PathBuf};

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
pub async fn rocket_main() {
    let cfg = config::load_config();
    let teps = Path::new("themes").join(&cfg.site.theme).join("templates");
    if !teps.exists() {
        eprintln!("Theme do not exists or broken.");
        std::process::exit(101);
    }
    let rc_figment = rocket::Config::figment().merge(("template_dir", teps.to_str()));
    rocket::custom(rc_figment)
        .attach(Template::fairing())
        .mount("/", routes![index, config_test])
        .mount("/static", routes![static_files])
        .launch()
        .await
        .unwrap();
}
