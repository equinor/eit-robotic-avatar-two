use common::SendMessage;

#[tokio::main]
async fn main() {
    let server = robot::setup().await;

    println!("Config from server: {:?}", server.config());

    let msg = SendMessage {
        topic: "robot".to_string(),
        msg_type: "hello".to_string(),
        payload: "world".to_string(),
    };
    server.send_message(msg).await.unwrap();
}
