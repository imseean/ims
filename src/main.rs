extern crate chrono;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate prettytable;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate uuid;
mod site;
fn main() {
    // let matches = clap_app!(myapp =>
    //     (version: "0.0.1")
    //     (author: "S W. <imseean@gmail.com>")
    //     (about: "My blog.")
    //     (@subcommand init =>
    //         (about: "Init a site.")
    //         (@arg path: -p --path "Set root for the new site.")
    //     )
    //     (@subcommand new =>
    //         (about: "Create a new content.")
    //         (@arg PATH: +required "Set new filename.")
    //         (@arg draft: -d --draft "Create as draft.")
    //     )
    //     (@subcommand generate =>
    //     )
    //     (@subcommand publish =>
    //     )
    //     (@subcommand server =>
    //     )
    // ).get_matches();load
    let site = site::Site::load("./site");
    site.list_content();
    // site.new_content("hello.md");
}
