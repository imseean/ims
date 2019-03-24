extern crate chrono;
extern crate clap;
extern crate colored;
extern crate handlebars;
extern crate iron;
#[macro_use]
extern crate log;
extern crate mount;
#[macro_use]
extern crate prettytable;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate cursive;
extern crate pulldown_cmark;
extern crate simplelog;
extern crate staticfile;
extern crate uuid;

mod app;
mod command;
mod command_app;
mod infrastructure;
mod model;

use std::fs::File;

use simplelog::*;

fn init_log() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, Config::default()).unwrap(),
        WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            File::create("msmusic.log").unwrap(),
        ),
    ])
    .unwrap();
}
fn main() {
    init_log();
    command_app::run();
}
