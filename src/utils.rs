use anyhow::{anyhow, Result};
use hmac::{digest::KeyInit, Hmac};
use jwt::{SigningAlgorithm, ToBase64};
use serde_json::json;
use sha2::Sha256;

use crate::{api::InvokeType, model::ModelType};

pub fn create_jwt_token(api_key: &str, expire: std::time::Duration) -> Result<String> {
    let sp = api_key.split('.').collect::<Vec<_>>();
    let api_key = *sp.first().ok_or_else(|| anyhow!("Invalid API key"))?;
    let secret = *sp.last().ok_or_else(|| anyhow!("Invalid API key"))?;

    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?;
    let exp = now + expire;

    let now_ts = now.as_millis();
    let exp_ts = exp.as_millis();

    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes())?;
    let header = json!(
        {"alg":"HS256","sign_type":"SIGN","typ":"JWT"}
    );

    let claims = json!({
        "api_key" : api_key,
        "exp" : exp_ts,
        "timestamp":now_ts
    });
    let header = header.to_base64()?;
    let claims = claims.to_base64()?;
    let signature = key.sign(&header, &claims)?;

    let token_string = [&*header, &*claims, &signature].join(".");
    Ok(token_string)
}

pub fn build_api_url(model: ModelType ,invoke_type: InvokeType) -> String {
    let api_base = std::env::var("ZHIPUAI_MODEL_API_URL")
        .unwrap_or_else(|_| crate::ZHIPUAI_MODEL_DEAULT_API_URL.to_string());
    let api_url = format!("{}/{}/{}", api_base, model, invoke_type);
    api_url
}
