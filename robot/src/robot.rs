use anyhow::Result;

use crate::{Config, server::Server};

pub struct Robot {

}
impl Robot {
    pub async fn install(config: &Config) -> Result<Robot> {
        
    }

    pub async fn run(&mut self, server:&mut Server, config: &Config) -> Result<Robot> {
        todo!()
    }

    pub async fn shutdown(self, server:&mut Server, config: &Config) -> Result<Robot> {
        todo!()
    }
}