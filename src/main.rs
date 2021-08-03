mod config;
mod markdown;
mod rcapp;

use clap::{App, Arg, SubCommand};
use std::net;
use std::str::FromStr;

#[macro_use]
extern crate rocket;

fn main() {
    let linkpress_app = App::new("LinkPress")
        .version("0.1.0")
        .author("AbrahumLink <307887491@qq.com>")
        .about("A static site generator by rust.")
        .subcommand(SubCommand::with_name("init").about("init a new site."))
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
        .subcommand(
            SubCommand::with_name("generate")
                .about("generate static file.")
                .alias("g"),
        );
    let mut help_app = linkpress_app.clone();
    let matches = linkpress_app.get_matches();

    if let Some(_) = matches.subcommand_matches("init") {
        println!("Init here");
    } else if let Some(s) = matches.subcommand_matches("serve") {
        let mut host: net::IpAddr = net::IpAddr::from([127, 0, 0, 1]);
        if let Some(value) = s.value_of("host") {
            if let Ok(ipvalue) = net::IpAddr::from_str(value) {
                host = ipvalue;
            }
        }
        let mut port: u16 = 4040;
        if let Some(value) = s.value_of("port") {
            if let Ok(u16value) = u16::from_str(value) {
                port = u16value;
            }
        }
        println!("{}:{}", host, port);
        rcapp::rocket_main();
    } else if let Some(m) = matches.subcommand_matches("new") {
        println!(
            "new page type:{},name:{}",
            m.value_of("type").unwrap(),
            m.value_of("name").unwrap()
        );
    } else if let Some(_) = matches.subcommand_matches("generate") {
        println!("generate here.");
    } else {
        help_app.print_help().unwrap();
    }
}
