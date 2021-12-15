// TODO should be replace by parsing a config file

use http::Uri;

const NAME_SERVICE_URI: &str = "http://127.0.0.1:2000";
const NETWORK_URI: &str = "http://127.0.0.1:1025";

pub struct Config {
    name_service: Uri,
    network: Uri,
}

impl Config {
    pub fn parse() -> Self {
        Self {
            name_service: NAME_SERVICE_URI.parse().unwrap(),
            network: NETWORK_URI.parse().unwrap(),
        }
    }

    pub fn name_service(&self) -> &Uri {
        &self.name_service
    }

    pub fn network(&self) -> &Uri {
        &self.network
    }
}
