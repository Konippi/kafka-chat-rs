use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() {
    let mut stdout = tokio::io::stdout();
    stdout.write(b"Welcome to kafka-chat-rs\n").await.unwrap();
}
