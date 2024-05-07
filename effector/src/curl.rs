use marine_rs_sdk::{marine, MountedBinaryResult};
use crate::utils::{check_url, inject_vault};
use curl_effector_imports as curl;
use eyre::{eyre, Result};
use curl_effector_imports::{CurlRequest,CurlResult,HttpHeader};

const CONNECT_TIMEOUT: usize = 4;

#[marine]
pub fn curl_post(
    request: CurlRequest,
    data_vault_path: &str,
    output_vault_path: &str,
) -> CurlResult {
    curl_post_impl(request, data_vault_path, output_vault_path).into()
}

fn curl_post_impl(
    request: CurlRequest,
    data_vault_path: &str,
    output_vault_path: &str,
) -> Result<String> {
    let url = check_url(request.url)?;
    let data_vault_path = inject_vault(data_vault_path)?;
    let output_vault_path = inject_vault(output_vault_path)?;
    let mut args = vec![
        String::from(url),
        String::from("-X"),
        String::from("POST"),
        String::from("--data"),
        format!("@{}", data_vault_path),
        String::from("-o"),
        output_vault_path,
    ];
    let mut headers = format_header_args(&request.headers);
    args.append(&mut headers);
    run_curl(args).map(|res| res.trim().to_string())
}


fn run_curl(mut cmd: Vec<String>) -> Result<String> {
    let mut default_arguments = vec![
        String::from("--connect-timeout"),
        format!("{}", CONNECT_TIMEOUT),
        String::from("--no-progress-meter"),
        String::from("--retry"),
        String::from("0"),
    ];
    cmd.append(&mut default_arguments);

    log::debug!("curl arguments: {:?}", cmd);
    let result = curl(cmd.clone());
    log::debug!("curl result: {:?}", result.stringify());

    result
        .into_std()
        .ok_or(eyre!("stdout or stderr contains non valid UTF8 string"))?
        .map_err(|e| eyre!("curl cli call failed \n{:?}: {}", cmd.join(" "), e))
}

fn format_header_args(headers: &[HttpHeader]) -> Vec<String> {
    let mut result = Vec::new();
    for header in headers {
        result.push("-H".to_string());
        result.push(format!("{}: {}", header.name, header.value))
    }
    result
}


#[marine]
#[host_import]
extern "C" {
    /// Execute provided cmd as a parameters of curl, return result.
    pub fn curl(cmd: Vec<String>) -> MountedBinaryResult;
}
