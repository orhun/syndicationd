use std::io;
use std::path::PathBuf;

use clap::Args;
use url::Url;

/// Import feed subscriptions
#[derive(Args, Debug)]
pub struct ImportCommand {
    /// input file. "-" means stdin
    #[arg(long, short = 'f', default_value = "-")]
    file: PathBuf,
}

impl ImportCommand {
    pub async fn run(self, endpoint: Url) -> i32 {
        0
    }

    async fn import<R: io::Read>(src: R) -> anyhow::Result<()> {
        // Read line
        // Parse subscription
        // emit subscribe api call
        // handle error
        Ok(())
    }
}
