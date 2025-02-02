use std::{collections::HashMap, ops::Deref, sync::LazyLock};

use rdkafka::{producer::FutureProducer, ClientConfig};

type ProducerConfig = HashMap<&'static str, &'static str>;
static PRODUCER_CONFIG: LazyLock<ProducerConfig> = LazyLock::new(|| {
    let mut config = HashMap::new();
    config.insert("queue.buffering.max.ms", "0");
    config.insert("message.timeout.ms", "5000");
    config
});

pub struct ProducerClient(FutureProducer);

impl Deref for ProducerClient {
    type Target = FutureProducer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ProducerClient {
    pub fn new(host: &str) -> anyhow::Result<Self> {
        let mut client_config = ClientConfig::new();

        client_config.set("bootstrap.servers", host);
        for (k, v) in PRODUCER_CONFIG.iter() {
            client_config.set(*k, *v);
        }
        let client = client_config.create()?;

        Ok(ProducerClient(client))
    }
}
