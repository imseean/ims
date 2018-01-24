extern crate chrono;
extern crate clap;
extern crate handlebars;
#[macro_use]
extern crate log;
#[macro_use]
extern crate prettytable;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate uuid;

mod model;
mod infrastructure;
mod command;

use log::LevelFilter;
use clap::{App, Arg, SubCommand};

use infrastructure::Logger;

static LOGGER: Logger = Logger {
    level: log::Level::Debug,
};

fn main() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(LevelFilter::Debug);

    let app = App::new("Ims")
        .version("1.0")
        .author("S W. <imseean@gmail.com>")
        .about("A simple static blog.")
        .version_short("v")
        .subcommand(
            SubCommand::with_name("new")
                .about("Create a new site.")
                .arg(
                    Arg::with_name("PATH")
                        .help("The root of the site. Default use the current directory.")
                        .required(true),
                )
                .display_order(0),
        )
        .subcommand(
            SubCommand::with_name("init")
                .about("Create a new site in the current directory.")
                .display_order(1),
        )
        .subcommand(
            SubCommand::with_name("status")
                .about("Show the status of the current site.")
                .display_order(2),
        )
        .subcommand(
            SubCommand::with_name("build")
                .about("Build the current site.")
                .display_order(3),
        )
        .subcommand(
            SubCommand::with_name("publish")
                .about("Publish the current site.")
                .display_order(4),
        )
        .subcommand(
            SubCommand::with_name("content")
                .about("Some commands about content.")
                .subcommand(
                    SubCommand::with_name("new")
                        .about("Create a new content.")
                        .arg(
                            Arg::with_name("PATH")
                                .help("File name of the new content.")
                                .required(true),
                        )
                        .arg(
                            Arg::with_name("draft")
                                .help("Create a new draft content.")
                                .short("d")
                                .long("draft"),
                        )
                        .display_order(0),
                )
                .subcommand(
                    SubCommand::with_name("list")
                        .about("List all contents.")
                        .display_order(1),
                )
                .display_order(5),
        );
    let matches = app.get_matches();
    if let None = matches.subcommand_name() {
        println!("{}", matches.usage());
    }

    if let Some(_) = matches.subcommand_matches("new") {
        if let Err(error) = command::new_site() {
            error!("{}", error);
        }
    };
    if let Some(matches) = matches.subcommand_matches("init") {
        let path = matches.value_of("PATH").unwrap_or(".");
        if let Err(error) = command::init_site(path) {
            error!("{}", error);
        }
    };
    if let Some(_) = matches.subcommand_matches("status") {
        if let Err(error) = command::show_site(".") {
            error!("{}", error);
        }
    };
    if let Some(_) = matches.subcommand_matches("build") {
        if let Err(error) = command::build_site(".") {
            error!("{}", error);
        }
    };
    if let Some(_) = matches.subcommand_matches("publish") {
        if let Err(error) = command::publish_site(".") {
            error!("{}", error);
        }
    };
    if let Some(matches) = matches.subcommand_matches("content") {
        let site = match command::load_site(".") {
            Ok(site) => site,
            Err(error) => {
                error!("{}", error);
                return;
            }
        };
        if let Some(matches) = matches.subcommand_matches("new") {
            let path = matches.value_of("PATH").unwrap_or(".");
            if let Err(error) = command::new_content(&site, path) {
                error!("{}", error);
            }
        }
        if let Some(_) = matches.subcommand_matches("list") {
            if let Err(error) = command::list_content(&site) {
                error!("{}", error);
            }
        }
    };
}
