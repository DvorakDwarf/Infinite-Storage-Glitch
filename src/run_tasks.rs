use crate::args::Arguments;

pub mod dislodge;
pub mod download;
pub mod embed;

pub async fn run_by_arguments(args: Arguments) -> anyhow::Result<()> {
    match args
        .command
        .expect("Command was not provided by the time run_by_arguments is used")
    {
        crate::args::Commands::Embed(args) => embed::run_embed(args).await,
        crate::args::Commands::Download(args) => download::run_download(args).await,
        crate::args::Commands::Dislodge(args) => dislodge::run_dislodge(args).await,
    }
}
