use regex::Regex;
use rust_warc::WarcReader;
use std::fs;
use std::io::BufReader;
use std::io::Read;
use std::sync::mpsc;
use std::thread;
use zstd::Decoder;

pub fn get_warc_items(url: String) -> impl Iterator<Item = String> {
    let (tx, rx) = mpsc::channel::<String>();

    let url = url.clone();
    thread::spawn(move || {
        let dict = get_dict(url.clone()).unwrap();

        let resp = reqwest::blocking::get(url.clone()).unwrap();

        let decoder = Decoder::with_dictionary(std::io::BufReader::new(resp), &dict).unwrap();

        let warc = WarcReader::new(std::io::BufReader::new(decoder));

        for next in warc {
            let item = next.unwrap();
            let Ok(body) = std::str::from_utf8(&item.content) else {
                continue;
            };
            tx.send(body.to_string()).unwrap();
        }

        println!("Finished processing {}", url);
    });

    rx.into_iter()
}

fn get_dict<'a>(url: String) -> Result<Vec<u8>, reqwest::Error> {
    let id = pastebin_url_to_dic_id(url);
    let path = format!("cache/{id}.zstdict");

    // Check if file exists in cache
    let exists = fs::metadata(path.clone()).is_ok();
    if exists {
        let bytes: Vec<u8> = fs::read(path.clone()).unwrap();
        return Ok(bytes);
    }

    let stream = reqwest::blocking::get(dictionary_url(id.clone())).unwrap();
    let mut decoder = Decoder::new(BufReader::new(stream)).unwrap();

    let mut buffer = Vec::new();
    decoder.read_to_end(&mut buffer).unwrap();

    // Ensure cache directory exists
    fs::create_dir_all("cache").unwrap();
    fs::write(path.clone(), &buffer).unwrap();

    Ok(buffer)
}

fn pastebin_url_to_dic_id(url: String) -> String {
    let re = Regex::new(r"\.(\d+)\.megawarc\.warc\.zst$").unwrap();
    let caps = re.captures(&url).unwrap();
    caps.get(1).unwrap().as_str().to_string()
}

fn dictionary_url(id: String) -> String {
    format!("https://archive.org/download/archiveteam_pastebin_dictionary_{id}/archiveteam_pastebin_dictionary_{id}.zstdict.zst")
}
