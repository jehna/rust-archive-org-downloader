use super::response::ArchiveOrgResponse;

pub fn advanced_search(collection: String, media_type: String, page: i64) -> ArchiveOrgResponse {
    let url =format!("https://archive.org/advancedsearch.php?q=collection:{collection}+mediatype:{media_type}&output=json&fl=identifier&rows=1000&page={page}&sort=-addeddate");
    reqwest::blocking::get(url)
        .unwrap()
        .json::<ArchiveOrgResponse>()
        .unwrap()
}
