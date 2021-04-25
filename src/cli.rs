#[allow(unused_imports)]
use log::*;

use clap::{Arg, App, ArgMatches, SubCommand};

/// Acts as both the definition for the structure of the CLI, as well as the entrance to it.
/// The CLI is based on the `clap` crate, and has all of the out of the box features that you would expect.
/// 
/// ```terminal
/// rpr -h
/// ```
pub fn process_args() -> ArgMatches<'static> {
    App::new("Reaper")
        .version(clap::crate_version!())
        .author("Ross Murray - @rossmurr4y")
        .about("A simple command-line utility to manage your git remotes.")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("A custom Reaper config file to use. Defaults to ~/reaper.toml")
            .takes_value(true))
        .arg(Arg::with_name("init")
            .short("i")
            .long("init")
            .help("Initialise a Reaper config file. If configuration already exists, existing content will suppliment defaults"))
        .arg(Arg::with_name("v")
            .short("v")
            .help(
r#"Logging verbosity: 

    : ERROR, WARN, INFO
-v  : + DEBUG
-vv : + TRACE

"#)
            .multiple(true)
            .use_delimiter(false))
        .arg(Arg::with_name("quiet")
            .short("q")
            .long("quiet")
            .help("Run quietly. Only errors will be reported"))
        .subcommand(SubCommand::with_name("add")
            .about("Add new configuration to your Reaper config file")
            .subcommand(SubCommand::with_name("remote")
                .about("Track a new remote repository.")
                .arg(Arg::with_name("name")
                    .short("n")
                    .long("name")
                    .help("A unique identifer for the remote's configuration")
                    .value_name("STRING")
                    .takes_value(true)
                    .required_unless("name_pos"))
                .arg(Arg::with_name("name_pos")
                    .help("Alias for --name")
                    .value_name("STRING")
                    .takes_value(true)
                    .index(1)
                    .conflicts_with("name"))
                .arg(Arg::with_name("description")
                    .short("d")
                    .long("description")
                    .help("A personal descriptor for the repository")
                    .value_name("STRING")
                    .takes_value(true))
                .arg(Arg::with_name("url")
                    .short("u")
                    .long("url")
                    .help("The URL of the remote repository")
                    .value_name("URL")
                    .takes_value(true))
                .arg(Arg::with_name("upstream_url")
                    .short("x")
                    .long("upstream")
                    .help("A URL for an `upstream` fork of the repository which you would like to track")
                    .value_name("URL")
                    .takes_value(true))
                .arg(Arg::with_name("branch")
                    .short("b")
                    .long("branch")
                    .help("The primary branch of the remote you wish to track")
                    .value_name("STRING")
                    .takes_value(true))
                .arg(Arg::with_name("path")
                    .short("p")
                    .long("path")
                    .help("A filepath within the remote repository to specific content you wish to track")
                    .value_name("PATH")
                    .takes_value(true))
            ))
        .get_matches()
}