# kafka-chat-rs

A simple chat application in Rust using Kafka as a message broker.
This is a simple example of how to use Kafka using the [rdkafka](https://github.com/fede1024/rust-rdkafka) crate.

![Demo](https://github.com/user-attachments/assets/2dd5feab-de3b-49fc-a211-91ca108df0e5)

## How to run

1. Start Kafka server using docker.

    ```bash
    $ docker compose up -d
    ```

2. Open multiple terminals and run the following command in each terminal.

    ```bash
    $ cargo run
    ```
