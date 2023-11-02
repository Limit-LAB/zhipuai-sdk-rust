use serde::{Deserialize, Serialize};
use super::*;
use serde_json::json;

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
