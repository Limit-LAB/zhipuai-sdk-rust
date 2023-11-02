use serde::{Deserialize, Serialize};
use super::*;
use serde_json::json;

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

impl Default for ChatGLMInvokeParam {
    fn default() -> Self {
        ChatGLMInvokeParam {
            top_p: 0.75,
            temperature: 0.5,
            request_id: None,
            return_type: None,
            glm_ref: None,
        }
    }
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
