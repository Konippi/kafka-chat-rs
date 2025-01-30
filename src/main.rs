use std::collections::HashMap;

use producer::Producer;
use rdkafka::{producer::FutureRecord, util::Timeout};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

mod producer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut stdout = tokio::io::stdout();
    let mut input_lines = BufReader::new(tokio::io::stdin()).lines();

    stdout.write(b"Welcome to kafka-chat-rs\n").await.unwrap();

    let mut producer_config = HashMap::new();
    producer_config.insert("queue.buffering.max.ms", "0");
    producer_config.insert("message.timeout.ms", "5000");
    let producer = Producer::new("localhost:9094", producer_config)?;

    loop {
        stdout.write(b"> ").await.unwrap();
        stdout.flush().await.unwrap();

        match input_lines.next_line().await.unwrap() {
            Some(line) => {
                producer
                    .client
                    .send(
                        FutureRecord::<(), _>::to("chat").payload(&line),
                        Timeout::Never,
                    )
                    .await
                    .expect("Failed to produce message");
            }
            None => break,
        }
    }

    Ok(())
}
