use zhipuai_sdk_rust::api::GLMSession;

#[tokio::main]
async fn main() {
    println!("chatglm_6b");
    let invoke_param = zhipuai_sdk_rust::model::InvokeParam {
        model_type: zhipuai_sdk_rust::model::ModelType::ChatGLM6b,
        prompt: serde_json::json!([{"role": "user", "content": "人工智能"}]),
        top_p: 0.7_f64,
        temperature: 0.9_f64,
    };
    GLMSession::invoke(invoke_param).await.unwrap();
}