mod list;
mod open;
pub mod shortlinks;

use saucer::{clap, Parser};
use serde::Serialize;

use crate::{command::RoverOutput, Result};

#[derive(Debug, Serialize, Parser)]
pub struct Docs {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Serialize, Parser)]
pub enum Command {
    /// List all available docs links
    List(list::List),

    /// Open a docs link
    Open(open::Open),
}

impl Docs {
    pub fn run(&self) -> Result<RoverOutput> {
        match &self.command {
            Command::List(command) => command.run(),
            Command::Open(command) => command.run(),
        }
    }
}
