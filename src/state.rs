use log::*;
use clap::ArgMatches;
use chrono::Utc;

/// Definitions for the Reaper CLI subcommands, arguments and associated configuration.
pub mod cli;

/// State is the current in-memory Reaper runtime state.
pub struct State<'a> {
    /// The system time that the State was initialized
    pub process_start: String,
    /// All inputs that were passed at the time of initialization
    pub inputs: ArgMatches<'a>,
    /// The queue of Task's requring actioning.
    pub queue: Option<Vec<Task>>,
}

impl State<'_> {
        /// Initialise the State
        pub fn initialize() -> Self {

            // Retrieval of the CLI params/args specified
            let cli_matches = cli::process_args();

            State {
                process_start: Utc::now().to_rfc3339(),
                inputs: cli_matches,
                queue: Some(Vec::new()),
            }
        }
    
        /// Appends a new Task into the State's queue for processing.
        pub fn queue_task(task: Task) -> Self {
            todo!();
        }
}

/// Task's are passed to a State's queue for processing.
pub struct Task {
    todo:()
}