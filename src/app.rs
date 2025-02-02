use std::sync::{Arc, Mutex};

use rdkafka::{consumer::Consumer, producer::FutureRecord, util::Timeout, Message};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, Stdout};

use crate::{consumer_client::ConsumerClient, producer_client::ProducerClient};

pub struct App {
    producer_client: ProducerClient,
    consumer_client: ConsumerClient,
    stdout: Arc<Mutex<Stdout>>,
}

impl App {
    pub fn new(host: &str, topics: &[&str]) -> anyhow::Result<Self> {
        let producer_client = ProducerClient::new(host)?;
        let consumer_client: ConsumerClient = ConsumerClient::new(host)?;
        consumer_client.subscribe(topics)?;

        Ok(Self {
            producer_client,
            consumer_client,
            stdout: Arc::new(Mutex::new(tokio::io::stdout())),
        })
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        self.write_message(b"Welcome to kafka-chat-rs\n").await?;
        let mut input_lines = BufReader::new(tokio::io::stdin()).lines();

        loop {
            self.write_message(b"> ").await?;

            tokio::select! {
                message = self.consumer_client.recv() => {
                    let message = message?.detach();
                    if let Some(payload) = message.payload() {
                        self.write_message(payload).await?;
                        self.write_message(b"\n").await?;
                    }
                }
                line = input_lines.next_line() => {
                    match line {
                        Ok(Some(line)) => {
                            self.send_message(line).await?;
                        }
                        _ => break
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn write_message(&mut self, payload: &[u8]) -> anyhow::Result<()> {
        let mut stdout = self.stdout.lock().unwrap();
        stdout.write_all(payload).await?;
        stdout.flush().await?;
        Ok(())
    }

    pub async fn send_message(&self, message: String) -> anyhow::Result<()> {
        self.producer_client
            .send(
                FutureRecord::<(), _>::to("chat").payload(&message),
                Timeout::Never,
            )
            .await
            .map_err(|(err, _)| err)?;

        Ok(())
    }
}
