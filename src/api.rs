use crate::{error::SessionError, model::InvokeParam, utils};
use eventsource_client::{Client, SSE};
use futures::TryStreamExt;
use std::fmt::Display;

pub enum InvokeType {
    SSE,
}

impl Display for InvokeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvokeType::SSE => write!(f, "sse-invoke"),
        }
    }
}

pub struct GLMSession {}

impl GLMSession {
    pub async fn invoke(invoke_param: InvokeParam) -> Result<(), SessionError> {
        let api_key = std::env::var("ZHIPUAI_API_KEY").unwrap();
        let model_type = invoke_param.model_type;
        let url = crate::utils::build_api_url(model_type, InvokeType::SSE);
        let token = utils::create_jwt_token(
            &api_key,
            std::time::Duration::from_secs(10000),
        )
        .unwrap();
        let client = eventsource_client::ClientBuilder::for_url(&url)
            .unwrap()
            .header("Authorization", &token)
            .unwrap()
            .header("Content-Type", "application/json; charset=UTF-8")
            .unwrap()
            .method("POST".to_string())
            .body(serde_json::to_string(&invoke_param.clone().json()).unwrap())
            .build();

        let mut stream = Box::pin(client.stream())
            .map_ok(|event| match event {
                SSE::Comment(comment) => println!("{:?}", comment),
                SSE::Event(evt) => println!("{}", evt.data),
            })
            .map_err(|e| println!("error streaming events: {:?}", e));

        while let Ok(Some(_)) = stream.try_next().await {}
        Ok(())
    }
}
