#[tokio::main]
async fn main() {
    robot::setup().await;

    println!("Hello, world!");
}
