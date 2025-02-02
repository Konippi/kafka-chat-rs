use std::{collections::HashMap, ops::Deref, sync::LazyLock};

use rdkafka::{consumer::StreamConsumer, ClientConfig};
use uuid::Uuid;

type ConsumerConfig = HashMap<&'static str, &'static str>;
static CONSUMER_CONFIG: LazyLock<ConsumerConfig> = LazyLock::new(|| {
    let mut config = HashMap::new();
    config.insert("queue.buffering.max.ms", "0");
    config.insert("message.timeout.ms", "5000");
    config
});

pub struct ConsumerClient(StreamConsumer);

impl Deref for ConsumerClient {
    type Target = StreamConsumer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ConsumerClient {
    pub fn new(host: &str) -> anyhow::Result<Self> {
        let mut client_config = ClientConfig::new();

        client_config.set("bootstrap.servers", host);
        client_config.set("group.id", format!("chat-{}", Uuid::new_v4()));
        for (k, v) in CONSUMER_CONFIG.iter() {
            client_config.set(*k, *v);
        }
        let client = client_config.create()?;

        Ok(ConsumerClient(client))
    }
}
