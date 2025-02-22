use saucer::{clap, Parser};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Serialize, Deserialize, Parser)]
pub struct ProfileOpt {
    /// Name of configuration profile to use
    #[clap(long = "profile", default_value = "default")]
    #[serde(skip_serializing)]
    pub profile_name: String,
}

impl Display for ProfileOpt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.profile_name)
    }
}
