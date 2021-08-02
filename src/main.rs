mod config;
mod markdown;
mod rcapp;

use clap::{App, Arg, SubCommand};

#[macro_use]
extern crate rocket;

fn main() {
    let linkpress_app = App::new("LinkPress")
        .version("0.1.0")
        .author("AbrahumLink <307887491@qq.com")
        .about("A static site generator by rust.")
        .subcommand(SubCommand::with_name("init").about("init a new site."))
        .subcommand(SubCommand::with_name("serve").about("start built-in server."))
        .subcommand(
            SubCommand::with_name("new")
                .about("create a new page.")
                .arg(
                    Arg::with_name("type")
                        .short("t")
                        .long("type")
                        .required(true)
                        .takes_value(true)
                        .help("the type of the new page"),
                )
                .arg(
                    Arg::with_name("name")
                        .short("n")
                        .long("name")
                        .required(true)
                        .takes_value(true)
                        .help("the name of the new page"),
                ),
        )
        .subcommand(SubCommand::with_name("generate").about("generate static file."))
        .get_matches();
    if let Some(_) = linkpress_app.subcommand_matches("serve") {
        rcapp::rocket_main();
    } else if let Some(_) = linkpress_app.subcommand_matches("s") {
        rcapp::rocket_main();
    } else if let Some(m) = linkpress_app.subcommand_matches("new") {
        println!(
            "new page type:{},name:{}",
            m.value_of("type").unwrap(),
            m.value_of("name").unwrap()
        );
    }
}
