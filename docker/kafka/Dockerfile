FROM bitnami/kafka:3.9.0

WORKDIR /opt/bitnami/kafka

RUN bin/kafka-storage.sh random-uuid > /tmp/CLUSTER_ID && \
    export KAFKA_KRAFT_CLUSTER_ID=$(cat /tmp/CLUSTER_ID) && \
    bin/kafka-storage.sh format -t "$KAFKA_KRAFT_CLUSTER_ID" -c ./config/kraft/server.properties

ENV KAFKA_KRAFT_CLUSTER_ID_FILE=/tmp/CLUSTER_ID
