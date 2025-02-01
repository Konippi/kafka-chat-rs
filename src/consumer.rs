use std::collections::HashMap;

use rdkafka::{consumer::StreamConsumer, ClientConfig};
use uuid::Uuid;

pub struct Consumer {
    pub client: StreamConsumer,
}

impl Consumer {
    pub fn new<S: Into<String>>(host: S, config: HashMap<S, S>) -> anyhow::Result<Self> {
        let mut client_config = ClientConfig::new();

        client_config.set("bootstrap.servers", host);
        client_config.set("group.id", format!("chat-{}", Uuid::new_v4()));
        for (k, v) in config {
            client_config.set(&k.into(), &v.into());
        }
        let client = client_config.create()?;

        Ok(Consumer { client })
    }
}
