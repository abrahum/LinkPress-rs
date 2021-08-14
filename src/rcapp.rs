use crate::config;
use crate::markdown;
use crate::utils;
use clap::ArgMatches;
use rocket::fs::NamedFile;
use rocket::tokio;
use rocket_dyn_templates::Template;
use std::net;
use std::path::{Path, PathBuf};
use std::str::FromStr;

// root routes
#[get("/")]
async fn index() -> Template {
    let lp_config = config::load_config();
    let md_string = String::new();
    let cwd = PathBuf::from(".");
    let dir_tree = utils::build_dir(&cwd);
    let mut context = markdown::build_pagedata("index", md_string, &lp_config);
    context.index = Some(dir_tree.build_index("posts"));
    Template::render("index", context)
}

#[get("/favicon.ico")]
async fn favicon() -> Option<NamedFile> {
    NamedFile::open(Path::new("favicon.ico")).await.ok()
}

// static routes
#[get("/<file..>")]
async fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}

// theme routes
#[get("/js/<file..>")]
async fn js_files(file: PathBuf) -> Option<NamedFile> {
    let lp_config = config::load_config();
    let js_dir = Path::new("themes").join(&lp_config.site.theme).join("js");
    NamedFile::open(js_dir.join(file)).await.ok()
}

#[get("/css/<file..>")]
async fn css_files(file: PathBuf) -> Option<NamedFile> {
    let lp_config = config::load_config();
    let css_dir = Path::new("themes").join(&lp_config.site.theme).join("css");
    NamedFile::open(css_dir.join(file)).await.ok()
}

// tags routes
#[get("/")]
async fn tags_index() -> Template {
    let lp_config = config::load_config();
    let md_string = String::new();
    let cwd = PathBuf::from(".");
    let dir_tree = utils::build_dir(&cwd);
    let mut context = markdown::build_pagedata("index", md_string, &lp_config);
    let tags = utils::build_tag_vec(&dir_tree);
    context.tags_index = tags;
    Template::render(String::from("tags"), context)
}

#[get("/<tag_name>")]
async fn tags_pages(tag_name: &str) -> Template {
    let lp_config = config::load_config();
    let md_string = String::new();
    let cwd = PathBuf::from(".");
    let dir_tree = utils::build_dir(&cwd);
    let mut context = markdown::build_pagedata("index", md_string, &lp_config);
    context.index = Some(dir_tree.build_tags_index().get(tag_name).unwrap().clone());
    Template::render(String::from("index"), context)
}

// types routes
#[get("/<type_>", rank = 2)]
async fn types_index(type_: &str) -> Template {
    let lp_config = config::load_config();
    let md_string = String::new();
    let cwd = PathBuf::from(".");
    let dir_tree = utils::build_dir(&cwd);
    let mut context = markdown::build_pagedata("index", md_string, &lp_config);
    context.index = Some(dir_tree.build_index(type_));
    Template::render("index", context)
}

#[get("/<type_>/<name>", rank = 2)]
async fn types_page(type_: &str, name: &str) -> Template {
    let mut template_name = type_;
    let lp_config = config::load_config();
    let md_string = utils::get_page(&type_, name).unwrap();
    let context = markdown::build_pagedata(name, md_string, &lp_config);
    if let Some(t) = &context.front_matter.template {
        template_name = &t;
    }
    Template::render(String::from(template_name), context)
}

// main function of rocket
#[tokio::main]
async fn tokio_run(template_dir: &str, host: net::IpAddr, port: u16) {
    let rc_figment = rocket::Config::figment()
        .merge(("template_dir", template_dir))
        .merge(("address", host))
        .merge(("port", port));
    rocket::custom(rc_figment.clone())
        .attach(Template::fairing())
        .mount(
            "/",
            routes![index, types_index, types_page, favicon, js_files, css_files],
        )
        .mount("/tags", routes![tags_index, tags_pages])
        .mount("/static", routes![static_files])
        .launch()
        .await
        .unwrap();
}

// set config of rocket and run main function
pub fn serve(s: &ArgMatches) {
    let lp_config = config::load_config();
    let mut host: net::IpAddr = lp_config.serve.host;
    let mut port: u16 = lp_config.serve.port;
    if let Some(value) = s.value_of("host") {
        if let Ok(h) = net::IpAddr::from_str(value) {
            host = h;
        }
    }
    if let Some(value) = s.value_of("port") {
        if let Ok(p) = u16::from_str(value) {
            port = p;
        }
    }
    let template_dir = Path::new("themes")
        .join(&lp_config.site.theme)
        .join("templates");

    if !template_dir.exists() {
        eprintln!("Theme do not exists or broken.");
        std::process::exit(101);
    }
    tokio_run(template_dir.to_str().unwrap(), host, port);
}
