# kafka-chat-rs

A simple chat application in Rust using Kafka as a message broker.
This is a simple example of how to use Kafka using the [rdkafka](https://github.com/fede1024/rust-rdkafka) crate.

https://github.com/user-attachments/assets/ab3c1f32-d900-47e5-9c3c-959bd790a053

## How to run

1. Start Kafka server using docker.

    ```bash
    $ docker compose up -d
    ```

2. Open multiple terminals and run the following command in each terminal.

    ```bash
    $ cargo run
    ```
