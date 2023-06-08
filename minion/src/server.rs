use common::RobotRegister;
use reqwest::{header, Client, RequestBuilder, Url};

use crate::config::Config;

pub struct Server {
    base_url: Url,
    client: Client,
}

impl Server {
    pub async fn connect(config: Config) -> Server {
        let mut headers = header::HeaderMap::new();
        let mut auth_value =
            header::HeaderValue::from_str(&("Bearer ".to_owned() + &config.token)).unwrap();
        auth_value.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, auth_value);

        let client = Client::builder().default_headers(headers).build().unwrap();
        let base_url = config.server_url;

        let register = RobotRegister {
            name: "Minion".to_string(),
            network_interfaces: get_networking_interfaces(),
        };

        client
            .post(base_url.join("api/robot/register").unwrap())
            .json(&register)
            .send()
            .await
            .unwrap();

        Server { base_url, client }
    }

    pub fn get(&self, path: &str) -> RequestBuilder {
        self.client.get(self.base_url.join(path).unwrap())
    }
}

use common::Interface;
use network_interface::{NetworkInterface, NetworkInterfaceConfig};

pub fn get_networking_interfaces() -> Vec<Interface> {
    let network_interfaces = NetworkInterface::show().unwrap_or_default();
    network_interfaces
        .into_iter()
        .flat_map(into_interface)
        .collect()
}

fn into_interface(i: NetworkInterface) -> Vec<Interface> {
    i.addr
        .iter()
        .map(|addr| Interface {
            name: i.name.clone(),
            ip: addr.ip().to_string(),
            broadcast: addr.broadcast().map(|b| b.to_string()).unwrap_or_default(),
            netmask: addr.netmask().map(|n| n.to_string()).unwrap_or_default(),
            mac: i.mac_addr.as_ref().cloned().unwrap_or_default(),
        })
        .collect()
}
