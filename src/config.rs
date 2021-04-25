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
/// [[repository]]
/// 
/// name = "My Example Repository"
/// description = "Singleton configured remote repository"
/// url = "https://github.com/org/repo.git"
/// branch = "main"
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// remote git repositories
    pub repository: Option<Vec<Repository>>,
}

impl Config {

    /// Deserialise a TOML file (as a string) into Config
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mystr = r#"
    /// [[repository]]
    /// name = "My Example Repository"
    /// description = "Singleton configured remote repository"
    /// "#;
    /// let test = Config::from_toml(mystr.to_string());
    /// assert_eq!(test.repository.is_some(), true);
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
            repository: None
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
/// Typically built by instantiating a Remote and adding attributes using the builder pattern.
/// 
/// # Example
/// 
/// ```toml
/// [[repository]]
/// 
/// name = "repository"
/// description = "My helpful descriptor"
/// url = "https://github.com/examplefork/rpr.git"
/// upstream = "https://github.com/rossmurr4y/rpr.git"
/// branch = "main"
/// path = "docs/"
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
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
}

/// A Repository builder for the Repository struct. 
/// 
/// Allows precise control over the instantiation and attributes defined for a Repository.
/// 
/// Examples
/// 
/// ```
/// let ex = Remote::new(String::from("Example001"))
///     .description(String::from("This is an example remote I am tracking"))
///     .url(String::from("http://github.com/rossmurr4y/rpr"))
///     .upstream(String::from("http://github.com/some_org/rpr"))
///     .branch("development")
///     .path("/")
///     .create();
/// assert_eq!(ex.url, String::from("http://github.com/rossmurr4y/rpr"));
/// ```
#[derive(Debug)]
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
}

impl Remote {

    /// Instantiate a new Remote.
    pub fn new(name: String) -> Remote {
        Remote {
            name,
            description: None,
            url: None,
            upstream: None,
            branch: None,
            path: None,
        }
    }

    /// Set the user-readible description for the Remote.
    /// Note that the description has no impact on functionality.
    pub fn description(mut self, desc: String) -> Self {
        self.description = Some(desc.to_string());
        self
    }

    /// Set the URL that the Remote can be accessed on.
    /// Must be either pre-authenticated, or otherwise able to authenticate via other means i.e ENV.
    pub fn url(mut self, url: String) -> Self {
        self.url = Some(url.to_string());
        self
    }

    /// Configure the Remote with an upstream fork's URL.
    /// Must be either pre-authenticated, or otherwise able to authenticate via other means i.e ENV.
    pub fn upstream(mut self, upstream: String) -> Self {
        self.upstream = Some(upstream.to_string());
        self
    }

    /// Set the primary Remote branch to be monitored and tracked.
    pub fn branch(mut self, branch: String) -> Self {
        self.branch = Some(branch.to_string());
        self
    }

    /// Set the path within the Remote to the content you wish to track.
    pub fn path(mut self, path: String) -> Self {
        self.path = Some(path.to_string());
        self
    }

    /// Creates the Repository with the options configgured so far on the Remote
    pub fn create(self) -> Repository {
        Repository {
            name: self.name,
            description: self.description,
            url: self.url,
            upstream: self.upstream,
            branch: self.branch,
            path: self.path,
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::config::{Config, Remote};
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn config_from_toml_str_no_remote() {
        let s = r#"
        "#;
        let r = Config::from_toml(s.to_string());
        assert_eq!(r.unwrap().repository.is_none(), true);
    }

    #[test]
    fn config_from_toml_str_single_remote() {
        let s = r#"
        [[repository]]
        name = "My Example Repository"
        description = "Singleton configured remote repository"
        "#;
        let r = Config::from_toml(s.to_string());
        assert_eq!(r.unwrap().repository.is_some(), true);
    }

    #[test]
    fn config_from_toml_str_multi_remote() {
        let s = r#"
        [[repository]]
        name = "repository"
        description = "My helpful descriptor"
        url = "https://github.com/examplefork/rpr.git"
        upstream = "https://github.com/rossmurr4y/rpr.git"
        branch = "main"
        path = "docs/"
        
        [[repository]]
        name = "My Example Repository"
        description = "Singleton configured remote repository"
        "#;
        let r = Config::from_toml(s.to_string());
        assert_eq!(r.unwrap().repository.is_some(), true);
    }

    #[test]
    fn config_from_filepath() {

        let mut file = NamedTempFile::new().unwrap();

        write!(file, r#"
            [[repository]]
            name = "repository"
            description = "My helpful descriptor"
            url = "https://github.com/examplefork/rpr.git"
            upstream = "https://github.com/rossmurr4y/rpr.git"
            branch = "main"
            path = "docs/"
            
            [[repository]]
            name = "My Example Repository"
            description = "Singleton configured remote repository"
            "#);
        let path = &file.into_temp_path();
        let config = Config::from_filepath(&path);
        assert_eq!(config.is_ok(), true);
    }

    #[test]
    fn remote_from_builder_pattern() {
        let name = String::from("Example001");
        let ex = Remote::new(name)
            .description(String::from("This is an example remote I am tracking"))
            .url(String::from("http://github.com/rossmurr4y/rpr"))
            .upstream(String::from("http://github.com/some_org/rpr"))
            .branch(String::from("development"))
            .path(String::from("/"))
            .create();
        assert_eq!(ex.url, Some(String::from("http://github.com/rossmurr4y/rpr")));
    }

}