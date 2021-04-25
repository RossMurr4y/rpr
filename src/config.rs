use std::path::Path;
use std::io::{Result};
use std::fs;

#[allow(unused_imports)]
use log::*;

use serde::{Serialize, Deserialize};


/// Reaper reads and stores configuration as TOML. It is either
/// user-provided or created new by RPR. 
/// 
/// The `Config` struct is the root definition for the Reaper configuration file.
/// 
/// # Example
/// 
/// The following example shows a single Remote configured.
/// 
/// ```toml
/// [[remote]]
/// 
/// name = "My Example Repository"
/// description = "Singleton configured remote repository"
/// url = "https://github.com/org/repo.git"
/// branch = "main"
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// remote git repositories
    pub remote: Option<Vec<Remote>>,
}

impl Config {

    /// Deserialise a TOML file (as a string) into Config
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mystr = r#"
    /// [[remote]]
    /// name = "My Example Repository"
    /// description = "Singleton configured remote repository"
    /// "#;
    /// let test = Config::from_toml(mystr.to_string());
    /// assert_eq!(test.remote.is_some(), true);
    /// ```
    pub fn from_toml (input: String) -> Result<Self> {
        Ok(toml::from_str(&input).unwrap())
    }

    /// Deserialise a file from a filepath into Config
    pub fn from_filepath(input: &Path) -> Result<Self> {
        let file_content = fs::read_to_string(&input)?;
        Config::from_toml(file_content)
    }

    /// Serialise a Config struct into a file. The file will be created if it doesn't already exist.
    pub fn to_file(filepath: &Path, config: Config) -> Result<()> {
        let output_str = toml::to_string_pretty(&config);
        fs::write(filepath, output_str.unwrap())
    }

    /// Initiate a new Reaper configuration file at the provided path.
    pub fn init(filepath: &Path) -> Result<()> {
        let default_conf = Config {
            remote: None
        };
        // Create all parent directories necessary
        fs::create_dir_all(filepath)?;
        println!("testing, {:#?}", filepath);
        fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(filepath)?;
        Config::to_file(filepath, default_conf)
    }
}

/// Remote repository configuration to be used by RPR
/// 
/// Defines the possible configuration of the Repository object
/// within the TOML configuration file. The properties are used
/// to define a remote repository to monitor, update, or otherwise
/// interact with.
/// 
/// # Example
/// 
/// ```toml
/// [[remote]]
/// 
/// name = "repository"
/// description = "My helpful descriptor"
/// url = "https://github.com/examplefork/rpr.git"
/// upstream = "https://github.com/rossmurr4y/rpr.git"
/// branch = "main"
/// path = "docs/"
/// org = "exampleorg"
/// platform = "github" 
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct Remote {
    /// A human-readable identifier for the remote. Mandatory.
    pub name: String,
    /// A brief descriptor for the remote - purely for identification purposes only.
    pub description: Option<String>,
    /// The URL of the remote.
    pub url: Option<String>,
    /// An URL of a upstream fork of the remote.
    pub upstream: Option<String>,
    /// The primary branch of the remote.
    pub branch: Option<String>,
    /// A path within the repository to the target content.
    pub path: Option<String>,
    /// An organisation to which this remote belongs.
    pub org: Option<String>,
    /// The git platform of the remote repository.
    pub platform: Option<String>,
}

#[cfg(test)]
mod tests {

    use crate::config::{Config};
    use std::io::Write;
    use tempfile::NamedTempFile;


    #[test]
    fn config_from_toml_str_no_remote() {
        let s = r#"
        "#;
        let r = Config::from_toml(s.to_string());
        assert_eq!(r.unwrap().remote.is_none(), true);
    }

    #[test]
    fn config_from_toml_str_single_remote() {
        let s = r#"
        [[remote]]
        name = "My Example Repository"
        description = "Singleton configured remote repository"
        "#;
        let r = Config::from_toml(s.to_string());
        assert_eq!(r.unwrap().remote.is_some(), true);
    }

    #[test]
    fn config_from_toml_str_multi_remote() {
        let s = r#"
        [[remote]]
        name = "repository"
        description = "My helpful descriptor"
        url = "https://github.com/examplefork/rpr.git"
        upstream = "https://github.com/rossmurr4y/rpr.git"
        branch = "main"
        path = "docs/"
        org = "exampleorg"
        platform = "github"
        
        [[remote]]
        name = "My Example Repository"
        description = "Singleton configured remote repository"
        "#;
        let r = Config::from_toml(s.to_string());
        assert_eq!(r.unwrap().remote.is_some(), true);
    }

    #[test]
    fn config_from_filepath() {

        let mut file = NamedTempFile::new().unwrap();

        write!(file, r#"
            [[remote]]
            name = "repository"
            description = "My helpful descriptor"
            url = "https://github.com/examplefork/rpr.git"
            upstream = "https://github.com/rossmurr4y/rpr.git"
            branch = "main"
            path = "docs/"
            org = "exampleorg"
            platform = "github"
            
            [[remote]]
            name = "My Example Repository"
            description = "Singleton configured remote repository"
            "#);
        let path = &file.into_temp_path();
        let config = Config::from_filepath(&path);
        assert_eq!(config.is_ok(), true);
    }

}