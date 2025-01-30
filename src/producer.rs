use std::collections::HashMap;

use rdkafka::{producer::FutureProducer, ClientConfig};

pub struct Producer {
    pub client: FutureProducer,
}

impl Producer {
    pub fn new<S: Into<String>>(host: S, config: HashMap<S, S>) -> anyhow::Result<Self> {
        let mut client_config = ClientConfig::new();

        client_config.set("bootstrap.servers", host);
        for (k, v) in config {
            client_config.set(&k.into(), &v.into());
        }
        let client = client_config.create()?;

        Ok(Producer { client })
    }
}
