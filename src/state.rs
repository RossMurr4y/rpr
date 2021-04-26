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

/// Task's are passed to a State's queue for processing. They follow the builder-pattern, and are typically created through Action's rather than created themselves.
pub struct Task {
    /// A system-managed identifier
    id: String,
    /// The priority of this Action
    priority: Option<i16>,
    /// The command that the Action is going to trigger
    cmd: String,
    /// Arguments to be provided to the command
    args: Vec<String>,
}


/// Action's are a builder for Task's. They should first be created with `new()`, assembled as necessary and then translated into a Task with `()`.
pub struct Action {
    /// A system-managed identifier
    id: String,
    /// The priority of this Action
    priority: Option<i16>,
    /// The command that the Action is going to trigger
    cmd: String,
    /// Arguments to be provided to the command
    args: Vec<String>,
}

impl Action {
    /// Creates a new Action
    pub fn new(name: String) -> Self {        
        // todo!("Add in validation of the command name being passed in");
        // todo!("Replace the stand-in id value with a system-managed one")
        Action {
            id: String::from("action_id"),
            priority: None,
            cmd: name,
            args: Vec::new(),
        }
    }

    /// Sets an Action's priority. Lower priority Action's will be processed sooner.
    pub fn priority(mut self, p: i16) -> Self {
        self.priority = Some(p);
        self
    }

    /// Assigns the Action a single argument.
    pub fn with_arg(mut self, a: String) -> Self {
        self.args.push(a);
        self
    }

    /// Assigns a collection of arguments to an Action.
    pub fn with_args(mut self, args: Vec<String>) -> Self {
        for i in args {
            self.args.push(i);
        }
        self
    }

    /// Builds the Action into a Task.
    pub fn ready(mut self) -> Task {
        Task {
            id: self.id,
            priority: self.priority,
            cmd: self.cmd,
            args: self.args,
        }
    }
}