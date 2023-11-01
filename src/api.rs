use crate::{
    error::APIError,
    models::{InvokeParam, Model},
    utils,
};
use eventsource_client::{Client, SSE, BoxStream};
use serde::{Serialize, Deserialize};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum InvokeType {
    Invoke,
    AsyncInvoke,
    SSE,
}

impl Display for InvokeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvokeType::SSE => write!(f, "sse-invoke"),
            InvokeType::Invoke => write!(f, "invoke"),
            InvokeType::AsyncInvoke => write!(f, "async-invoke"),
        }
    }
}

pub async fn sse_invoke<T: InvokeParam>(model: &Model, invoke_param: &T) -> Result<BoxStream<eventsource_client::Result<SSE>>, APIError>{
    let api_key = std::env::var("ZHIPUAI_API_KEY").expect("ZHIPUAI_API_KEY is not set");
    let url = crate::utils::build_api_url(model, InvokeType::SSE);
    let token = utils::create_jwt_token(&api_key, std::time::Duration::from_secs(10000)).unwrap();
    let client = eventsource_client::ClientBuilder::for_url(&url)
        .unwrap()
        .header("Authorization", &token)
        .unwrap()
        .header("Content-Type", "application/json; charset=UTF-8")
        .unwrap()
        .method("POST".to_string())
        .body(serde_json::to_string(&invoke_param.json()).unwrap())
        .build();

    Ok(client.stream())
}

pub async fn http_invoke<T: InvokeParam>(
    model: &Model,
    invoke_param: &T,
) -> Result<serde_json::Value, APIError> {
    let api_key = std::env::var("ZHIPUAI_API_KEY").expect("ZHIPUAI_API_KEY is not set");
    let url = crate::utils::build_api_url(model, InvokeType::Invoke);
    let token = utils::create_jwt_token(&api_key, std::time::Duration::from_secs(10000)).unwrap();
    let client = reqwest::Client::new();
    if let Ok(resp) = client
        .post(&url)
        .header("Authorization", &token)
        .header("Content-Type", "application/json; charset=UTF-8")
        .body(serde_json::to_string(&invoke_param.json()).unwrap())
        .send()
        .await
    {
        let status = resp.status();
        let body = resp.text().await.expect("Failed to get response body");
        if status.is_success() {
            let json: serde_json::Value = serde_json::from_str(&body).unwrap(); // TODO: handle error
            return Ok(json)
        } else {
            return Err(APIError::APIInvokeError)
        }
    } else {
        return Err(APIError::APIInvokeError)
    }
}
