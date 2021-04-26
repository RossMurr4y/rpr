use log::*;
use clap::ArgMatches;
use chrono::Utc;

/// Definitions for the Reaper CLI subcommands, arguments and associated configuration.
pub mod cli;

#[derive(Debug)]
/// State is the current in-memory Reaper runtime state.
pub struct State<'a> {
    /// The system time that the State was initialized
    pub process_start: String,
    /// All inputs that were passed at the time of initialization
    pub inputs: ArgMatches<'a>,
    /// The queue of Task's requring actioning.
    pub queue: Vec<Task>,
}

impl State<'_> {
    /// Initialise the State
    pub fn initialize() -> Self {

        /// Retrieval of the CLI params/args specified
        let cli_matches = cli::process_args();

        let mut state = State {
            process_start: Utc::now().to_rfc3339(),
            inputs: cli_matches,
            queue: Vec::new(),
        };

        // Stage initialisation Tasks
        // Set the logging level
        let log_level = Action::new(String::from("set_log_level")).priority(100).ready();
        state.queue_task(log_level);

        // Evaluate the rpr configuration file
        let rpr_conf = Action::new(String::from("set_state_from_rpr_conf")).priority(200).ready();
        &state.queue_task(rpr_conf);

        state
    }

    /// Appends a new Task into the State's queue for processing.
    pub fn queue_task(&mut self, task: Task) -> &Self {
        self.queue.push(task);
        self
    }
}

#[derive(Debug)]
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
    pub fn ready(self) -> Task {
        Task {
            id: self.id,
            priority: self.priority,
            cmd: self.cmd,
            args: self.args,
        }
    }
}