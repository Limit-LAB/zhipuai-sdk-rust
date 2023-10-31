use serde_json::json;

#[derive(Debug, Clone, Copy)]
pub enum ModelType {
    ChatGLM130b,
    ChatGLM6b,
}

impl std::fmt::Display for ModelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModelType::ChatGLM130b => write!(f, "chatglm_130b"),
            ModelType::ChatGLM6b => write!(f, "chatglm_6b"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct InvokeParam {
    pub model_type: ModelType,
    pub prompt: serde_json::Value,
    pub top_p: f64,
    pub temperature: f64,
}

impl InvokeParam {
    pub fn json(self) -> serde_json::Value {
        json!({
            "prompt": self.prompt,
            "top_p": self.top_p,
            "temperature": self.temperature,
        })
    }
}