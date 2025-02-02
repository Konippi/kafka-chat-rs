#!/bin/bash -eu

for TOPIC in $KAFKA_TOPICS; do
    echo "Creating topic: $TOPIC"

    /opt/bitnami/kafka/bin/kafka-topics.sh \
        --bootstrap-server kafka:9092 \
        --create \
        --if-not-exists \
        --topic "$TOPIC" \
        --replication-factor 1 \
        --partitions 1
done
