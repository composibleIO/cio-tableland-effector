use serde::{Deserialize, Serialize};
use marine_rs_sdk::marine;


#[derive(Debug, Serialize, Deserialize)]
pub struct TLCreds {
    key: String,
    domain: String
}

#[marine]
#[derive(Debug, Serialize, Deserialize)]
pub struct TLRequest {
    pub table: String,
    pub sql_query: String,
    pub content: String,
    pub optimistic: bool
}

#[marine]
#[derive(Debug, Serialize, Deserialize)]
pub struct TLQuery {
    pub query: String
}
