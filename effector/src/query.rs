use curl_effector_imports::{CurlRequest,CurlResult,HttpHeader};
use cio_response_types::{AMResponse};
use std::fs;
use std::path::PathBuf;
use chrono::Utc;

use crate::curl;
use crate::utils::{check_url, inject_vault};

pub fn run(url: String, body: &String) -> AMResponse {

    let h = HttpHeader {
        name: "Content-Type".to_string(),
        value: "application/json".to_string()
    };
 
    let source_path = vault_path("tl_body");
    let target_path = vault_path("tl_response");
    let _ = fs::write(PathBuf::from(source_path.clone()), body);

    let request = CurlRequest {
        url: url.clone(),    
        headers: vec![h]
    };

    let response = curl::curl_post(request, &source_path, &target_path.clone());

    let timestamp = Utc::now().timestamp_millis();
    let cp = crate::get_call_parameters();

    if response.success {

        let result = fs::read_to_string(target_path.clone()).unwrap();

        return AMResponse {
            success: true,
            result_raw: result.clone(),
            result: result,
            timestamp,
            host_id: cp.host_id
        }
    }

    else {
        
        return AMResponse {
            success: false,
            result_raw: String::from(""),
            result: response.error,
            timestamp,
            host_id: cp.host_id
            
        }
    }
}

// todo: replace with fn in utils
fn vault_path(filename: &str) -> String {
    let cp = crate::get_call_parameters();
    format!("/tmp/vault/{}-{}/{}", cp.particle.id, cp.particle.token, filename)
}