mod axumapp;
mod cli;
mod config;
mod deployer;
mod generator;
mod logger;
mod markdown;
mod utils;

use clap::{App, Arg, SubCommand};

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    let linkpress_app = App::new("LinkPress")
        .version("0.1.0")
        .author("AbrahumLink <307887491@qq.com>")
        .about("A static site generator by rust.")
        .subcommand(
            SubCommand::with_name("init").about("Init a new site.").arg(
                Arg::with_name("name")
                    .short("n")
                    .long("name")
                    .required(false)
                    .takes_value(true)
                    .help("The Linkpress dir name"),
            ),
        )
        .subcommand(
            SubCommand::with_name("serve")
                .about("Start built-in server.")
                .alias("s")
                .arg(
                    Arg::with_name("host")
                        .short("h")
                        .long("host")
                        .takes_value(true)
                        .required(false)
                        .help("The host of server"),
                )
                .arg(
                    Arg::with_name("port")
                        .short("p")
                        .long("port")
                        .takes_value(true)
                        .required(false)
                        .help("The port of server"),
                ),
        )
        .subcommand(
            SubCommand::with_name("new")
                .alias("n")
                .about("Create a new page.")
                .arg(
                    Arg::with_name("type")
                        .short("t")
                        .long("type")
                        .required(false)
                        .takes_value(true)
                        .help("The type of the new page"),
                )
                .arg(
                    Arg::with_name("name")
                        .short("n")
                        .long("name")
                        .required(true)
                        .takes_value(true)
                        .help("The name of the new page"),
                ),
        )
        .subcommand(
            SubCommand::with_name("generate")
                .about("Generate static file.")
                .alias("g"),
        )
        .subcommand(
            SubCommand::with_name("deploy")
                .about("Deploy your website wit git (to-do)")
                .alias("d"),
        );
    let mut help_app = linkpress_app.clone();
    let matches = linkpress_app.get_matches();

    if let Some(i) = matches.subcommand_matches("init") {
        let name = i.value_of("name");
        cli::init(name);
    } else if let Some(s) = matches.subcommand_matches("serve") {
        axumapp::tokio_run(s);
    } else if let Some(n) = matches.subcommand_matches("new") {
        cli::new(n.value_of("type"), n.value_of("name").unwrap());
    } else if let Some(_) = matches.subcommand_matches("generate") {
        generator::generator();
    } else if let Some(_) = matches.subcommand_matches("deploy") {
        deployer::deployer();
    } else {
        help_app.print_help().unwrap();
    }
}
