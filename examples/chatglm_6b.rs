use futures::TryStreamExt;
use zhipuai_sdk_rust::models::InvokeMeta;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    println!("chatglm_6b");
    let prompt = serde_json::json!([{"role": "user", "content": "人工智能"}]);
    let invoke_param = zhipuai_sdk_rust::models::chatglm::ChatGLMInvokeParam {
        request_id: None,
        return_type: None,
        glm_ref: None,
        top_p: 0.7_f64,
        temperature: 0.9_f64,
    };
    let model = zhipuai_sdk_rust::models::Model::ChatGLM6b;
    let invoke_meta = InvokeMeta {
        prompt,
        invoke_param,
    };

    let result = model.invoke(invoke_meta.clone()).await;
    println!("{}", result);
    let mut stream = model.invoke_sse(invoke_meta).await;
    loop {
        let event = stream.try_next().await;
        match event {
            Ok(Some(event)) => println!("{:?}", event),
            Ok(None) => break,
            Err(eventsource_client::Error::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
