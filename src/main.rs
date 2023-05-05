pub mod archive_org;
pub mod commands;
pub mod warc_utils;
pub mod zip_utils;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    DownloadPastebin(commands::download_pastebin::Args),
    LinkShortenerUrls(commands::link_shortener_urls::Args),
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::DownloadPastebin(args) => commands::download_pastebin::download_pastebin(args),
        Commands::LinkShortenerUrls(args) => {
            commands::link_shortener_urls::link_shortener_urls(args)
        }
    }
}
