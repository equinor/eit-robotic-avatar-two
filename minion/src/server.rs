use std::{sync::Arc, thread};

use anyhow::Result;
use common::{Tracking, Head, Drive};
use parking_lot::Mutex;
use reqwest::{blocking::Client, Url};

use crate::{config::LocalConfig};

#[derive(Clone)]
pub struct Server {
    http: Client,
    baseurl: Url,
    tracking: Arc<Mutex<Tracking>>,
}

impl Server {
    pub fn new(config: &LocalConfig) -> Result<Self> {
        let client = Client::builder()
            .danger_accept_invalid_certs(true) // Nesery evil for now!!!!
            .use_rustls_tls()
            .build()?;

        Ok(Server {
            http: client,
            baseurl: config.server.clone(),
            tracking: Arc::default(),
        })
    }

    /// Starts sever in a different tread.
    pub fn start(&self) {
        let mut server = self.clone();
        thread::spawn(move || loop {
            if let Err(error) = server.get_tracking() {
                eprintln!("Server request error: {}", error)
            }
        });
    }

    pub fn head(&self) -> Head {
        let tracking = self.tracking.lock();
        tracking.head
    }

    pub fn drive(&self) -> Drive {
        let tracking = self.tracking.lock();
        tracking.drive
    }

    fn get_tracking(&mut self) -> Result<()> {
        let server_tracking: Tracking = self.http.get(self.baseurl.join("/minion/tracking")?)
            .send()?
            .json()?;

        *self.tracking.lock() = server_tracking;
        Ok(())
    }
}