use reqwest::blocking::Client;
use crate::Error;
use crate::api::{B2Auth};
use crate::handle_b2error_kinds;

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
struct DeleteFileVersionBody<'a> {
    file_name: &'a str,
    file_id: &'a str,
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[serde(rename_all = "camelCase")]
/// Result object from [b2_delete_file_version](fn.b2_delete_file_version.html)
pub struct DeleteFileVersionResult {
    pub file_name: String,
    pub file_id: String,
}

/// <https://www.backblaze.com/b2/docs/b2_delete_file_version.html>
pub fn b2_delete_file_version<T: AsRef<str>, Q: AsRef<str>>(client: &Client, auth: &B2Auth, file_name: T, file_id: Q) -> Result<DeleteFileVersionResult, Error> {
    let req_body = serde_json::to_string(&DeleteFileVersionBody {
        file_name: file_name.as_ref(),
        file_id: file_id.as_ref(),
    }).unwrap();

    let resp = match client.post(&auth.api_url_for("b2_delete_file_version"))
        .header(reqwest::header::AUTHORIZATION, &auth.authorization_token)
        .body(req_body)
        .send() {
        Ok(v) => v,
        Err(e) => return Err(Error::ReqwestError(e))
    };
    if !resp.status().is_success() {
        return Err(Error::from_response(resp))
    }

    let response_string = resp.text().unwrap();
    let deserialized: DeleteFileVersionResult = match serde_json::from_str(&response_string) {
        Ok(v) => v,
        Err(_e) => {
            eprintln!("{:?}", response_string);
            return Err(handle_b2error_kinds(&response_string))
        }
    };
    Ok(deserialized)
}