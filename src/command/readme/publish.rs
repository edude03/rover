use saucer::{clap, Parser};
use serde::Serialize;

use crate::command::RoverOutput;
use crate::options::{GraphRefOpt, ProfileOpt};
use crate::utils::client::StudioClientConfig;
use crate::utils::color::Style;
use crate::utils::parsers::FileDescriptorType;
use crate::Result;

use rover_client::operations::readme::publish::{self, ReadmePublishInput};

#[derive(Debug, Serialize, Parser)]
pub struct Publish {
    #[clap(flatten)]
    graph: GraphRefOpt,

    #[clap(flatten)]
    profile: ProfileOpt,

    /// The file upload as the README. You can pass `-` to use stdin instead of a file.
    #[clap(long, short = 's')]
    #[serde(skip_serializing)]
    file: FileDescriptorType,
}

impl Publish {
    pub fn run(&self, client_config: StudioClientConfig) -> Result<RoverOutput> {
        let client = client_config.get_authenticated_client(&self.profile)?;
        let graph_ref = self.graph.graph_ref.to_string();
        eprintln!(
            "Publishing README for {} using credentials from the {} profile.",
            Style::Link.paint(&graph_ref),
            Style::Command.paint(&self.profile.profile_name)
        );

        let new_readme = self
            .file
            .read_file_descriptor("README", &mut std::io::stdin())?;
        tracing::debug!("Uploading \n{}", &new_readme);

        let publish_response = publish::run(
            ReadmePublishInput {
                graph_ref: self.graph.graph_ref.clone(),
                readme: new_readme,
            },
            &client,
        )?;

        Ok(RoverOutput::ReadmePublishResponse {
            graph_ref: self.graph.graph_ref.clone(),
            new_content: publish_response.new_content,
            last_updated_time: publish_response.last_updated_time,
        })
    }
}
