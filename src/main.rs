mod cli;
mod config;
mod markdown;
mod rcapp;

use clap::{App, Arg, SubCommand};

#[macro_use]
extern crate rocket;

fn main() {
    let linkpress_app = App::new("LinkPress")
        .version("0.1.0")
        .author("AbrahumLink <307887491@qq.com>")
        .about("A static site generator by rust.")
        .subcommand(
            SubCommand::with_name("init").about("init a new site.").arg(
                Arg::with_name("name")
                    .short("n")
                    .long("name")
                    .required(false)
                    .help("the Linkpress dir name"),
            ),
        )
        .subcommand(
            SubCommand::with_name("serve")
                .about("start built-in server.")
                .alias("s")
                .arg(
                    Arg::with_name("host")
                        .short("h")
                        .long("host")
                        .takes_value(true)
                        .required(false)
                        .help("the host of server"),
                )
                .arg(
                    Arg::with_name("port")
                        .short("p")
                        .long("port")
                        .takes_value(true)
                        .required(false)
                        .help("the port of server"),
                ),
        )
        .subcommand(
            SubCommand::with_name("new")
                .alias("n")
                .about("create a new page.")
                .arg(
                    Arg::with_name("type")
                        .short("t")
                        .long("type")
                        .required(false)
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
        .subcommand(
            SubCommand::with_name("generate")
                .about("generate static file.")
                .alias("g"),
        );
    let mut help_app = linkpress_app.clone();
    let matches = linkpress_app.get_matches();

    if let Some(i) = matches.subcommand_matches("init") {
        let name = i.value_of("name");
        cli::init(name);
    } else if let Some(s) = matches.subcommand_matches("serve") {
        rcapp::serve(s);
    } else if let Some(n) = matches.subcommand_matches("new") {
        cli::new(n.value_of("type"), n.value_of("name").unwrap());
    } else if let Some(_) = matches.subcommand_matches("generate") {
        println!("generate here.");
    } else {
        help_app.print_help().unwrap();
    }
}
