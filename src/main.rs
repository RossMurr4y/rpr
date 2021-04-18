#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

//! # Reaper (rpr)
//! 
//! Reaper is a simple command-line utility to manage your git remotes.
//! 
//! > Note: Reaper is still under (active) development. 
//! > The features below may be partially or wholly unimplemented.
//! 
//! # Overview
//! 
//! For projects that span numerous repositories it can be difficult to
//! keep each repository up to date. 
//! 
//! Feature development in such an environment has some challenges:
//! 
//! - New features often span repositories, each requiring a feature branch
//! - Pivoting to other work requires re-evaluating the correct branch for them all
//! - It's quite easy to forget to rebase all related repositories, leading to trivial mistakes
//! - Re-basing in general is often neglected until a PR is attempted
//! - Merged branches remain around locally to cause confusion instead of being pruned
//! - Not to mention potential upstream repositories with breaking changes!
//! 
//! # What Reaper does
//! 
//! Reaper aims to address these issues by performing the things we so easily forget:
//! 
//! - centralising _your_ configuration of repositories _you_ care about
//! - keeping them all up-to-date with other branches and remotes
//! - taking out the trash by removing local branches that are now merged
//! - establishing workspaces for quick pivots across branches and repositories
//! - and feature profiles for simultaneous feature branch creation across repositories
//! 
//! # What Reaper Does Not Do
//! 
//! Reaper is not intended to be a replacement or simplification of git. 
//! 
//! Whilst some of the capabilities of git may be surfaced, Reaper's mission
//! is to help users orchestrate and navigate and maintain their local workspace.

use clap::{Arg, App};

/// Contains definitions for the Reaper configuration as well as interactions with it.
pub mod config;

fn main() {
    let _matches = App::new("Reaper")
        .version("0.0.0")
        .author("Ross Murray - @rossmurr4y")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("A custom Documancy config file to use. Defaults to ~/reaper.toml")
            .takes_value(true))
        .get_matches();
}