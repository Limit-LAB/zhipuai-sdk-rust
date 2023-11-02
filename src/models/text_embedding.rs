use serde::{Deserialize, Serialize};
use super::*;
use serde_json::json;


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
