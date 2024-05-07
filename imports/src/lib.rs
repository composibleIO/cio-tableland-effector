pub use curl_effector_types::*;
use marine_rs_sdk::marine;
use cio_response_types::AMResponse;

mod types;

// should this be a separate thing? 
// howe to 'export' 
use types::{TLReq,TLQuery};


#[marine]
#[module_import("cio_tableland_effector")]
extern "C" {
    // insert rows into table
    pub fn tl_insert(gateway: String, tl_request: &TLReq) -> AMResponse;
   
    // query
    pub fn tl_query(gateway: String, tl_query: &TLQuery) -> AMResponse;    
}
