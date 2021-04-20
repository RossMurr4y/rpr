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
/// [[remotes]]
/// 
/// name = "My Example Repository"
/// description = "Singleton configured remote repository"
/// url = "https://github.com/org/repo.git"
/// branch = "main"
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub remotes: Option<Vec<Remote>>,
}

impl Config {

    /// Deserialise a TOML file (as a string) into Config
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mystr = r#"
    /// [[remotes]]
    /// name = "My Example Repository"
    /// description = "Singleton configured remote repository"
    /// "#;
    /// let mut test = Config::from_toml(mystr.to_string());
    /// println!("got me a: {:#?}", test);
    /// ```
    pub fn from_toml (input: String) -> Self {
        toml::from_str(&input).unwrap()
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
/// [[remotes]]
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
    pub name: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub upstream: Option<String>,
    pub branch: Option<String>,
    pub path: Option<String>,
    pub org: Option<String>,
    pub platform: Option<String>,
}