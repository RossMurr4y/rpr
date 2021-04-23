use clap::{Arg, App, ArgMatches};

/// Acts as both the definition for the structure of the CLI, as well as the entrance to it.
/// The CLI is based on the `clap` crate, and has all of the out of the box features that you would expect.
/// 
/// ```terminal
/// rpr -h
/// ```
pub fn process_args() -> ArgMatches<'static> {
    App::new("Reaper")
        .version("0.0.0")
        .author("Ross Murray - @rossmurr4y")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("A custom Documancy config file to use. Defaults to ~/reaper.toml")
            .takes_value(true))
        .get_matches()
}