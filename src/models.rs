use eventsource_client::{BoxStream, SSE};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::api;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Model {
    ChatGLM130b,
    ChatGLM6b,
    ChatGLMTurbo,
    CharacterGLM,
    TextEmbedding,
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
        api::invoke(self, &body).await.unwrap()
    }

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

impl<T: InvokeParam> InvokeMeta<T> {
    pub fn request_body(&self) -> serde_json::Value {
        let mut json = self.invoke_param.json();
        json["prompt"] = self.prompt.clone();
        json
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlmRef {
    pub enable: bool,
    pub query: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatGLMInvokeParam {
    pub top_p: f64,
    pub temperature: f64,
    pub request_id: Option<String>,
    pub return_type: Option<ReturnType>,
    pub glm_ref: Option<GlmRef>,
}

impl InvokeParam for ChatGLMInvokeParam {
    fn json(&self) -> serde_json::Value {
        let mut json = json!({
            "top_p": self.top_p,
            "temperature": self.temperature,
        });
        if let Some(request_id) = &self.request_id {
            json["request_id"] = json!(request_id);
        }
        if let Some(return_type) = &self.return_type {
            json["return_type"] = json!(return_type);
        }
        if let Some(glm_ref) = &self.glm_ref {
            json["glm_ref"] = json!({
                "enable": glm_ref.enable,
                "query": glm_ref.query
            });
        }
        json
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterGLMMeta {
    pub user_info: String,
    pub bot_info: String,
    pub bot_name: String,
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterGLMInvokeParam {
    pub meta: CharacterGLMMeta,
    pub request_id: Option<String>,
    pub return_type: Option<ReturnType>,
    pub incremental: Option<bool>,
}

impl InvokeParam for CharacterGLMInvokeParam {
    fn json(&self) -> serde_json::Value {
        let mut json = json!({
            "meta": {
                "user_info": self.meta.user_info,
                "bot_info": self.meta.bot_info,
                "bot_name": self.meta.bot_name,
            }
        });
        if let Some(user_name) = &self.meta.user_name {
            json["meta"]["user_name"] = json!(user_name);
        }
        if let Some(request_id) = &self.request_id {
            json["request_id"] = json!(request_id);
        }
        if let Some(return_type) = &self.return_type {
            json["return_type"] = json!(return_type);
        }
        if let Some(incremental) = &self.incremental {
            json["incremental"] = json!(incremental);
        }
        json
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextEmbeddingInvokeParam {
    pub request_id: Option<String>,
}

impl InvokeParam for TextEmbeddingInvokeParam {
    fn json(&self) -> serde_json::Value {
        let mut json = json!({});
        if let Some(request_id) = &self.request_id {
            json["request_id"] = json!(request_id);
        }
        json
    }
}
