use reqwest::blocking::Client;
use crate::Error;
use crate::api::{B2Auth, B2BucketType, BucketResult};
use crate::handle_b2error_kinds;

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
struct UpdateBucketBody<'a> {
    account_id: &'a str,
    bucket_id: &'a str,
    bucket_type: B2BucketType,
}

/// <https://www.backblaze.com/b2/docs/b2_update_bucket.html>
pub fn b2_update_bucket<T: AsRef<str>>(client: &Client, auth: &B2Auth, bucket_id: T, bucket_type: B2BucketType) -> Result<BucketResult, Error> {
    let req_body = serde_json::to_string(&UpdateBucketBody {
        account_id: &auth.account_id,
        bucket_id: bucket_id.as_ref(),
        bucket_type,
    }).unwrap();

    let resp = match client.post(&auth.api_url_for("b2_update_bucket"))
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
    let deserialized: BucketResult = match serde_json::from_str(&response_string) {
        Ok(v) => v,
        Err(_e) => {
            eprintln!("{:?}", response_string);
            return Err(handle_b2error_kinds(&response_string))
        }
    };
    Ok(deserialized)
}