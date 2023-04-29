use self::advanced_search::advanced_search;
use std::sync::mpsc;
use std::thread;
mod advanced_search;
mod get_archive_files;
mod response;
pub use get_archive_files::get_archive_files;

pub fn get_archive_org_ids(collection: String, media_type: String) -> impl Iterator<Item = String> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let mut page = 1;
        loop {
            let resp = advanced_search(collection.clone(), media_type.clone(), page);
            let num_docs = resp.response.docs.len();
            for doc in resp.response.docs {
                tx.send(doc.identifier).unwrap();
            }
            if resp.response.num_found <= resp.response.start + num_docs {
                break;
            }
            page += 1;
        }
        drop(tx);
    });

    rx.into_iter()
}
