#[tokio::main]
async fn main() {
    let server = robot::setup().await;

    println!("Config from server: {:?}", server.config());
}
