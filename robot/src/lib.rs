use common::{RobotConfig, RobotRegister};

pub async fn setup() -> RobotConfig {
    let register = RobotRegister{
        name: "robot".to_string()
    };

    let client = reqwest::Client::new();

    client.post("http://127.0.0.1:3000/api/robot/register")
    .json(&register)
    .send()
    .await.unwrap()
    .json()
    .await.unwrap()
}