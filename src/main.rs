use app::App;

mod app;
mod consumer_client;
mod logging;
mod producer_client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the logger
    logging::register_subscriber();

    // Run the chat app
    let mut app = App::new("localhost:9094", &["chat"])?;
    app.run().await
}
