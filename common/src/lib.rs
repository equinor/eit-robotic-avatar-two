use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RobotRegister {
    pub name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RobotConfig{

}
