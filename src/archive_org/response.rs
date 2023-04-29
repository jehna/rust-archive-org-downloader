use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ResponseParams {
    pub query: String,
    pub qin: String,
    pub fields: String,
    pub wt: String,
    pub rows: String,
    pub start: i64,
}

#[derive(Deserialize, Debug)]
pub struct ResponseHeader {
    pub status: i64,
    #[serde(rename = "QTime")]
    pub q_time: i64,
    pub params: ResponseParams,
}

#[derive(Deserialize, Debug)]
pub struct Reponse {
    #[serde(rename = "numFound")]
    pub num_found: usize,
    pub start: usize,
    pub docs: Vec<Doc>,
}

#[derive(Deserialize, Debug)]
pub struct Doc {
    pub identifier: String,
}

#[derive(Deserialize, Debug)]
pub struct ArchiveOrgResponse {
    #[serde(rename = "responseHeader")]
    pub response_header: ResponseHeader,
    pub response: Reponse,
}
