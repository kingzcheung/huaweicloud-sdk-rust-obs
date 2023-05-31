use std::time::Duration;

use crate::error::ObsError;

#[derive(Debug)]
pub struct Client {
    config: Config,
    http_client: reqwest::Client,
}

impl Client {
    pub fn new(access_key_id:&str,secret_access_key:&str,endpoint:&str)-> Result<Client, ObsError> { 
        ClientBuilder::new()
        .access_key_id(access_key_id)
        .secret_access_key(secret_access_key)
        .endpoint(endpoint)
        .build()
    }
     pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }
}

#[derive(Debug)]
pub struct ClientBuilder {
    config: Config,
}

#[derive(Debug)]
struct Config {
    access_key_id: String,
    secret_access_key: String,
    endpoint: String,
    is_secure: bool,
    region: String,
    timeout: Duration,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ClientBuilder {
    fn new() -> ClientBuilder {
        ClientBuilder {
            config: Config {
                access_key_id: "".into(),
                secret_access_key:"".into(),
                endpoint:"".into(),
                is_secure: false,
                region: "".into(),
                timeout: Duration::from_secs(3),
            },
        }
    }

    pub fn endpoint<S:ToString>(mut self, value:S)->ClientBuilder {
        self.config.endpoint = value.to_string();
        self
    }
    pub fn access_key_id<S:ToString>(mut self, value:S)->ClientBuilder {
        self.config.access_key_id = value.to_string();
        self
    }

    pub fn secret_access_key<S:ToString>(mut self, value:S)->ClientBuilder {
        self.config.secret_access_key = value.to_string();
        self
    }

    pub fn timeout(mut self, duration: Duration) ->ClientBuilder {
        self.config.timeout = duration;
        self
    }

    pub fn region<S:ToString>(mut self, value:S)->ClientBuilder {
        self.config.region = value.to_string();
        self
    }
    
    pub fn is_secure(mut self, value:bool) ->ClientBuilder {
        self.config.is_secure = value;
        self
    }

    

    fn build(self) -> Result<Client, ObsError> {
        let req_client = reqwest::ClientBuilder::new()
        .timeout(self.config.timeout)
        .build();
        Ok(
            Client {
                config: self.config,
                http_client:  req_client?,
           }
        )
    }
}
