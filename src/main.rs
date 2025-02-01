use std::collections::HashMap;

use producer::Producer;
use rdkafka::{consumer::Consumer, message::Message, producer::FutureRecord, util::Timeout};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

mod consumer;
mod producer;

const HOST: &str = "localhost:9094";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut stdout = tokio::io::stdout();
    let mut input_lines = BufReader::new(tokio::io::stdin()).lines();

    stdout.write(b"Welcome to kafka-chat-rs\n").await.unwrap();

    let mut producer_config = HashMap::new();
    producer_config.insert("queue.buffering.max.ms", "0");
    producer_config.insert("message.timeout.ms", "5000");
    let producer = Producer::new(HOST, producer_config)?.client;

    let mut consumer_config = HashMap::new();
    consumer_config.insert("enable.partition.eof", "false");
    let consumer = consumer::Consumer::new(HOST, consumer_config)?.client;
    consumer.subscribe(&["chat"])?;

    loop {
        stdout.write(b"> ").await.unwrap();
        stdout.flush().await.unwrap();

        tokio::select! {
            message = consumer.recv() => {
                let message  = message.expect("Failed to read message").detach();
                let payload = message.payload().unwrap();
                stdout.write(payload).await.unwrap();
                stdout.write(b"\n").await.unwrap();
            }
            line = input_lines.next_line() => {
                match line {
                    Ok(Some(line)) => {
                        producer.send(FutureRecord::<(), _>::to("chat")
                          .payload(&line), Timeout::Never)
                            .await
                            .expect("Failed to produce");
                    }
                    _ => break,
                }
            }
        }
    }

    Ok(())
}
