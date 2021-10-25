use http::Uri;

const NAME_SERVICE_URI: &str = "https://factory.c4dt.org/incubator/at2/demo/ns";
const NETWORK_URI: &str = "https://factory.c4dt.org/incubator/at2/demo/leader";

pub struct Config {
    name_service: Uri,
    network: Uri,
}

impl Config {
    pub fn parse() -> Result<Self, http::uri::InvalidUri> {
        Ok(Self {
            name_service: NAME_SERVICE_URI.parse()?,
            network: NETWORK_URI.parse()?,
        })
    }

    pub fn name_service(&self) -> &Uri {
        &self.name_service
    }

    pub fn network(&self) -> &Uri {
        &self.network
    }
}
