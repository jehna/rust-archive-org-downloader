use serde::Deserialize;
use std::sync::mpsc;
use std::thread;

#[derive(Debug, Deserialize)]
struct File {
    #[serde(rename = "@name")]
    name: String,
}

#[derive(Debug, Deserialize)]
struct Files {
    file: Vec<File>,
}

pub fn get_archive_files(archive_id: String) -> impl Iterator<Item = String> {
    let url = format!("https://archive.org/download/{archive_id}/{archive_id}_files.xml");
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || {
        let xml = reqwest::blocking::get(url).unwrap().text().unwrap();
        let files: Files = quick_xml::de::from_str(&xml).unwrap();
        for file in files.file {
            let file_name = file.name;
            tx.send(format!(
                "https://archive.org/download/{archive_id}/{file_name}",
            ))
            .unwrap();
        }
    });

    rx.into_iter()
}
