use eventsource_client::{BoxStream, SSE};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::api;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Model {
    ChatGLM130b(ChatGLMInvokeParam),
    ChatGLM6b(ChatGLMInvokeParam),
    ChatGLMTurbo(ChatGLMInvokeParam),
    CharacterGLM(CharacterGLMInvokeParam),
    TextEmbedding(TextEmbeddingInvokeParam),
}

pub enum InvokeResult {
    SSE(BoxStream<eventsource_client::Result<SSE>>),
    HTTP(serde_json::Value),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReturnType {
    JsonString,
    Text,
}

impl std::fmt::Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Model::ChatGLM130b(_) => write!(f, "chatglm_130b"),
            Model::ChatGLM6b(_) => write!(f, "chatglm_6b"),
            Model::ChatGLMTurbo(_) => write!(f, "chatglm_turbo"),
            Model::CharacterGLM(_) => write!(f, "character_glm"),
            Model::TextEmbedding(_) => write!(f, "text_embedding"),
        }
    }
}

impl Model {
    pub async fn invoke(&self, invoke_type: api::InvokeType) -> InvokeResult {
        match self {
            Model::ChatGLM130b(invoke_param) => match invoke_type {
                api::InvokeType::Invoke => {
                    InvokeResult::HTTP(api::http_invoke(self, invoke_param).await.unwrap()) // TODO: handle error
                }
                api::InvokeType::AsyncInvoke => todo!(),
                api::InvokeType::SSE => {
                    InvokeResult::SSE(api::sse_invoke(self, invoke_param).await.unwrap()) // TODO: handle error
                }
            },
            Model::ChatGLM6b(invoke_param) => match invoke_type {
                api::InvokeType::Invoke => {
                    InvokeResult::HTTP(api::http_invoke(self, invoke_param).await.unwrap()) // TODO: handle error
                },
                api::InvokeType::AsyncInvoke => todo!(),
                api::InvokeType::SSE => {
                    InvokeResult::SSE(api::sse_invoke(self, invoke_param).await.unwrap()) // TODO: handle error
                }
            },
            Model::ChatGLMTurbo(invoke_param) => match invoke_type {
                api::InvokeType::Invoke => {
                    InvokeResult::HTTP(api::http_invoke(self, invoke_param).await.unwrap()) // TODO: handle error
                },
                api::InvokeType::AsyncInvoke => todo!(),
                api::InvokeType::SSE => {
                    InvokeResult::SSE(api::sse_invoke(self, invoke_param).await.unwrap()) // TODO: handle error
                }
            },
            Model::CharacterGLM(invoke_param) => match invoke_type {
                api::InvokeType::Invoke => {
                    InvokeResult::HTTP(api::http_invoke(self, invoke_param).await.unwrap()) // TODO: handle error
                },
                api::InvokeType::AsyncInvoke => todo!(),
                api::InvokeType::SSE => {
                    InvokeResult::SSE(api::sse_invoke(self, invoke_param).await.unwrap()) // TODO: handle error
                }
            },
            Model::TextEmbedding(invoke_param) => match invoke_type {
                api::InvokeType::Invoke => {
                    InvokeResult::HTTP(api::http_invoke(self, invoke_param).await.unwrap()) // TODO: handle error
                },
                api::InvokeType::AsyncInvoke => todo!(),
                api::InvokeType::SSE => unimplemented!("SSE is not supported for TextEmbedding"),
            },
        }
    }
}

pub trait InvokeParam {
    fn json(&self) -> serde_json::Value;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlmRef {
    pub enable: bool,
    pub query: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatGLMInvokeParam {
    pub prompt: serde_json::Value,
    pub top_p: f64,
    pub temperature: f64,
    pub request_id: Option<String>,
    pub return_type: Option<ReturnType>,
    pub glm_ref: Option<GlmRef>,
}

impl InvokeParam for ChatGLMInvokeParam {
    fn json(&self) -> serde_json::Value {
        let mut json = json!({
            "prompt": self.prompt,
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
    pub prompt: serde_json::Value,
    pub meta: CharacterGLMMeta,
    pub request_id: Option<String>,
    pub return_type: Option<ReturnType>,
    pub incremental: Option<bool>,
}

impl InvokeParam for CharacterGLMInvokeParam {
    fn json(&self) -> serde_json::Value {
        let mut json = json!({
            "prompt": self.prompt,
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
    pub prompt: String,
    pub request_id: Option<String>,
}

impl InvokeParam for TextEmbeddingInvokeParam {
    fn json(&self) -> serde_json::Value {
        let mut json = json!({
            "prompt": self.prompt,
        });
        if let Some(request_id) = &self.request_id {
            json["request_id"] = json!(request_id);
        }
        json
    }
}
