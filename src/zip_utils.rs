use std::{
    io::{BufRead, BufReader},
    sync::mpsc,
    thread,
};

use xz::bufread::XzDecoder;
use zip::read::read_zipfile_from_stream;

pub fn stream_zip(url: String) -> impl Iterator<Item = String> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let mut resp = reqwest::blocking::get(url).unwrap();
        while let Ok(Some(file)) = read_zipfile_from_stream(&mut resp) {
            if !file.name().ends_with(".txt.xz") {
                continue;
            }

            let mut lines_reader = BufReader::new(XzDecoder::new(BufReader::new(file))).lines();

            while let Some(Ok(line)) = lines_reader.next() {
                if line.starts_with("#") || line.is_empty() {
                    continue;
                }
                let url = line.splitn(2, "|").nth(1).unwrap();
                tx.send(url.to_string()).unwrap();
            }
        }
    });

    rx.into_iter()
}
