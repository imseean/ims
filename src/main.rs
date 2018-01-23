extern crate chrono;
#[macro_use]
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

mod site;
mod infrastructure;

use log::LevelFilter;

use site::Site;
use infrastructure::Logger;

static LOGGER: Logger = Logger {
    level: log::Level::Debug,
};

fn main() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(LevelFilter::Debug);
    let matches = clap_app!(myapp =>
        (version: "0.0.1")
        (author: "S W. <imseean@gmail.com>")
        (about: "My blog.")
        (@subcommand init =>
            (about: "Init a site.")
            (@arg path: -p --path +takes_value "Set root for the new site.")
        )
        (@subcommand new =>
            (about: "Create a new content.")
            (@arg PATH: +required "Set new filename.")
            (@arg draft: -d --draft "Create as draft.")
        )
        (@subcommand generate =>
        )
        (@subcommand publish =>
        )
        (@subcommand server =>
        )
    ).get_matches();

    if let Some(matches) = matches.subcommand_matches("init") {
        let path = matches.value_of("path").unwrap_or(".");
        if let Err(error) = Site::new(path) {
            error!("{}", error);
        }
    };
}
