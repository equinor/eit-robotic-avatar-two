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
use network_interface::{Addr, NetworkInterface, NetworkInterfaceConfig};

pub fn get_networking_interfaces() -> Vec<Interface> {
    let network_interfaces = NetworkInterface::show().unwrap_or_default();
    network_interfaces.into_iter().map(into_interface).collect()
}

fn into_interface(i: NetworkInterface) -> Interface {
    let mut interface = Interface {
        name: i.name,
        ip: Default::default(),
        broadcast: Default::default(),
        netmask: Default::default(),
        mac: i.mac_addr.unwrap_or_default(),
    };

    if let Some(Addr::V4(addr)) = i.addr {
        interface.ip = addr.ip.to_string();
        if let Some(broadcast) = addr.broadcast {
            interface.broadcast = broadcast.to_string();
        }
        if let Some(netmask) = addr.netmask {
            interface.netmask = netmask.to_string();
        }
    }

    if let Some(Addr::V6(addr)) = i.addr {
        interface.ip = addr.ip.to_string();
        if let Some(broadcast) = addr.broadcast {
            interface.broadcast = broadcast.to_string();
        }
        if let Some(netmask) = addr.netmask {
            interface.netmask = netmask.to_string();
        }
    }
    interface
}
