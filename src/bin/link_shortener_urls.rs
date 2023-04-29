use clap::Parser;
use rayon::prelude::*;
use rust_archive_org_downloader::{
    archive_org::{get_archive_files, get_archive_org_ids},
    zip_utils::stream_zip,
};

#[derive(Parser, Debug)]
#[command(name = "Link shortener URL finder")]
#[command(author = "Jesse Luoto")]
#[command(version = "1.0")]
#[command(about = "Downloads link shortener URLs from archive.org (UrlteamWebCrawls project) and checks for matches", long_about = None)]
struct Args {
    #[arg(short, long)]
    regex: String,
}

fn main() {
    let args = Args::parse();
    let regex = regex::Regex::new(&args.regex).unwrap();

    get_archive_org_ids("UrlteamWebCrawls".to_string(), "software".to_string())
        .flat_map(|id| get_archive_files(id))
        .filter(|url| url.ends_with(".zip"))
        .par_bridge()
        .flat_map(|url| stream_zip(url).par_bridge())
        .filter(|result_url| regex.is_match(result_url))
        .for_each(|result_url| {
            println!("{}", result_url);
        });
}
