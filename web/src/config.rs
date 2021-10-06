use http::Uri;

const NAME_SERVICE_URI: &str = "https://factory.c4dt.org/incubator/at2/demo/ns";

pub struct Config {
    name_service: Uri,
}

impl Config {
    pub fn parse() -> Result<Self, http::uri::InvalidUri> {
        Ok(Self {
            name_service: NAME_SERVICE_URI.parse()?,
        })
    }

    pub fn name_service(&self) -> &Uri {
        &self.name_service
    }
}
