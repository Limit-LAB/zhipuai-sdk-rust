use futures::TryStreamExt;
use zhipuai_sdk_rust::api::InvokeType;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    println!("chatglm_6b");
    let invoke_param = zhipuai_sdk_rust::models::ChatGLMInvokeParam {
        request_id: None,
        return_type: None,
        glm_ref: None,
        prompt: serde_json::json!([{"role": "user", "content": "人工智能"}]),
        top_p: 0.7_f64,
        temperature: 0.9_f64,
    };
    let model = zhipuai_sdk_rust::models::Model::ChatGLM6b(invoke_param);

    let result = model.invoke(InvokeType::SSE).await;
    if let zhipuai_sdk_rust::models::InvokeResult::SSE(mut stream) = result {
        while let Some(event) = stream.try_next().await.unwrap() {
            println!("event: {:?}", event);
        }
    } else {
        
    }
}