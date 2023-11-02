use eventsource_client::{BoxStream, SSE};
use serde::{Deserialize, Serialize};
use crate::api;

pub mod chatglm;
pub mod characterglm;
pub mod text_embedding;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Model {
    ChatGLM130b,
    ChatGLM6b,
    ChatGLMTurbo,
    CharacterGLM,
    TextEmbedding,
}

impl Default for Model {
    fn default() -> Self {
        Model::ChatGLMTurbo
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReturnType {
    JsonString,
    Text,
}

impl std::fmt::Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Model::ChatGLM130b => write!(f, "chatglm_130b"),
            Model::ChatGLM6b => write!(f, "chatglm_6b"),
            Model::ChatGLMTurbo => write!(f, "chatglm_turbo"),
            Model::CharacterGLM => write!(f, "characterglm"),
            Model::TextEmbedding => write!(f, "text_embedding"),
        }
    }
}

impl Model {
    pub async fn invoke<T: InvokeParam>(&self, invoke_meta: InvokeMeta<T>) -> serde_json::Value {
        let body = invoke_meta.request_body();
        match api::invoke(self, &body).await {
            Ok(res) => res,
            Err(e) => {
                panic!("invoke error: {}", e)
            }
        }
        
    }
    
    // TODO: SSE has no meta
    pub async fn invoke_sse<T: InvokeParam>(&self, invoke_meta: InvokeMeta<T>) -> BoxStream<eventsource_client::Result<SSE>> {
        if self == &Model::TextEmbedding {
            unimplemented!("TextEmbedding does not support SSE invoke")
        }
        let body = invoke_meta.request_body();
        api::sse_invoke(self, &body).await.unwrap()
    }
}

pub trait InvokeParam {
    fn json(&self) -> serde_json::Value;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvokeMeta<T: InvokeParam> {
    pub prompt: serde_json::Value,
    pub invoke_param: T,
}

impl InvokeParam for serde_json::Value {
    fn json(&self) -> serde_json::Value {
        self.clone()
    }
}

impl<T: InvokeParam> InvokeMeta<T> {
    pub fn request_body(&self) -> serde_json::Value {
        let mut json = self.invoke_param.json();
        json["prompt"] = self.prompt.clone();
        json
    }
}