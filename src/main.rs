#[macro_use]
extern crate clap;
use clap::{Arg, App, SubCommand};
use clap::AppSettings;

mod command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Tesla Client Unofficial")
                    .version(crate_version!())
                    .author("Camille B. <git@camillebaronnet.fr>")
                    .about("Unofficial command line interface to ride over Tesla API.")
                    .arg(Arg::with_name("debug")
                        .short("d")
                        .help("Debug mode")
                    )
                    .subcommand(SubCommand::with_name("login")
                        .about("Log in to Tesla API")
                        .arg(Arg::with_name("password")
                            .short("p")
                            .value_name("PASSWORD")
                            .help("Password")
                        )
                        .arg(Arg::with_name("username")
                            .short("u")
                            .value_name("USERNAME")
                            .help("Username")
                        )
                    )
                    .subcommand(SubCommand::with_name("products")
                        .about("List products")
                    )
                    .setting(AppSettings::ArgRequiredElseHelp)
                    .get_matches();

    match matches.subcommand() {
        ("login", Some(cmd_matches)) => command::login(cmd_matches, &matches),
        ("products", Some(_)) => command::products(&matches),
        _ => command::not_found(),
    }
}
