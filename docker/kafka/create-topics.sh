#!/bin/sh -eu

docker exec -it kafka /opt/bitnami/kafka/bin/kafka-topics.sh \
      --bootstrap-server localhost:9094 \
      --create \
      --if-not-exists \
      --topic chat \
      --replication-factor 1 \
      --partitions 1
