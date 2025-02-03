use std::sync::{Arc, Mutex};

use rdkafka::{consumer::Consumer, producer::FutureRecord, util::Timeout, Message};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, Stdout};

use crate::{consumer_client::ConsumerClient, producer_client::ProducerClient};

const CHAT_TOPIC: &str = "chat";
const DEFAULT_USER_NAME: &str = "Unknown";

pub struct App {
    producer_client: ProducerClient,
    consumer_client: ConsumerClient,
    stdout: Arc<Mutex<Stdout>>,
    user_name: String,
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
            user_name: DEFAULT_USER_NAME.to_string(),
        })
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        self.write_prompt(b"Welcome to kafka-chat-rs\n").await?;
        let mut input_lines = BufReader::new(tokio::io::stdin()).lines();

        while self.user_name == DEFAULT_USER_NAME {
            self.write_prompt(b"Enter your name: ").await?;
            if let Some(name) = input_lines.next_line().await? {
                if !name.trim().is_empty() {
                    self.user_name = name.trim().to_string();
                }
            }
        }

        self.send_message("Joined chat!".to_string()).await?;
        self.write_prompt(b"> ").await?;

        loop {
            tokio::select! {
                message = self.consumer_client.recv() => {
                    match message {
                        Ok(msg) => {
                            let message = msg.detach();
                            let key = message.key().ok_or_else(|| anyhow::anyhow!("Missing key"))?;
                            if key != self.user_name.as_bytes() {
                                let payload = message.payload().ok_or_else(|| anyhow::anyhow!("Missing payload"))?;
                                self.display_message(key, payload).await?;
                            }
                        }
                        Err(err) => {
                            tracing::error!("Error receiving message: {:?}", err);
                        }
                    }
                }
                line = input_lines.next_line() => {
                    match line {
                        Ok(Some(line)) if !line.trim().is_empty() => {
                            self.send_message(line.trim().to_string()).await?;
                            self.write_prompt(b"> ").await?;
                        }
                        _ => break
                    }
                }
            }
        }

        Ok(())
    }

    async fn write_prompt(&self, payload: &[u8]) -> anyhow::Result<()> {
        let mut stdout = self.stdout.lock().unwrap();
        stdout.write_all(payload).await?;
        stdout.flush().await?;
        Ok(())
    }

    async fn display_message(&self, key: &[u8], payload: &[u8]) -> anyhow::Result<()> {
        if let (Ok(user), Ok(msg)) = (std::str::from_utf8(key), std::str::from_utf8(payload)) {
            self.write_prompt(format!("{}: {}", user, msg).as_bytes())
                .await?;
        } else {
            self.write_prompt(b"Invalid UTF-8 message\n").await?;
        }
        self.write_prompt(b"\n> ").await?;

        Ok(())
    }

    async fn send_message(&self, message: String) -> anyhow::Result<()> {
        self.producer_client
            .send(
                FutureRecord::<String, _>::to(CHAT_TOPIC)
                    .key(&self.user_name)
                    .payload(&message),
                Timeout::Never,
            )
            .await
            .map_err(|(err, _)| err)?;

        Ok(())
    }
}
