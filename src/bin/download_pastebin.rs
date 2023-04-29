use clap::Parser;
use rayon::prelude::*;
use rust_archive_org_downloader::{
    archive_org::{get_archive_files, get_archive_org_ids},
    warc_utils::get_warc_items,
};
use std::{
    ops::{AddAssign, Rem},
    sync::{Arc, Mutex},
};

#[derive(Parser, Debug)]
#[command(name = "Pastebin token finder")]
#[command(author = "Jesse Luoto")]
#[command(version = "1.0")]
#[command(
    about = "Downloads pastebin crawl archives from archive.org (archiveteam_pastebin project) and checks for matches.

You can control the number of threads by setting the `RAYON_NUM_THREADS` environment variable.",
    long_about = None
)]
struct Args {
    #[arg(short, long)]
    regex: String,
}
fn main() {
    let args = Args::parse();
    let regex = regex::Regex::new(&args.regex).unwrap();
    let thread_safe_counter = Arc::new(Mutex::new(0));

    get_archive_org_ids("archiveteam_pastebin".to_string(), "web".to_string())
        .flat_map(|id| get_archive_files(id))
        .filter(|filename| filename.ends_with(".megawarc.warc.zst"))
        .par_bridge()
        .for_each(|url| {
            for page in get_warc_items(url) {
                {
                    let mut count = thread_safe_counter.lock().unwrap();
                    count.add_assign(1);
                    if count.rem(100000) == 0 {
                        println!("Processed {} pages", count);
                    }
                }
                for line in page.lines() {
                    let tokens = regex.find_iter(line);

                    for token in tokens {
                        println!("{} found in: {}", token.as_str(), line);
                    }
                }
            }
        });
}
