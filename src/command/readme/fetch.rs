use saucer::{clap, Parser};
use serde::Serialize;

use crate::command::RoverOutput;
use crate::options::{GraphRefOpt, ProfileOpt};
use crate::utils::client::StudioClientConfig;
use crate::utils::color::Style;
use crate::Result;

use rover_client::operations::readme::fetch::{self, ReadmeFetchInput};

#[derive(Debug, Serialize, Parser)]
pub struct Fetch {
    #[clap(flatten)]
    graph: GraphRefOpt,

    #[clap(flatten)]
    profile: ProfileOpt,
}

impl Fetch {
    pub fn run(&self, client_config: StudioClientConfig) -> Result<RoverOutput> {
        let client = client_config.get_authenticated_client(&self.profile)?;
        let graph_ref = self.graph.graph_ref.to_string();

        eprintln!(
            "Fetching README for {} using credentials from the {} profile.",
            Style::Link.paint(&graph_ref),
            Style::Command.paint(&self.profile.profile_name)
        );
        let readme = fetch::run(
            ReadmeFetchInput {
                graph_ref: self.graph.graph_ref.clone(),
            },
            &client,
        )?;
        Ok(RoverOutput::ReadmeFetchResponse {
            graph_ref: self.graph.graph_ref.clone(),
            content: readme.content,
            last_updated_time: readme.last_updated_time,
        })
    }
}
