# Streaming skeleton (#43)

In-process `StreamBus`. Topics named by `SystemTwin`. Invalid records go to a dead-letter list. `checkpoint` / `restore` is the restart test.

Not Kafka. Not Flink. Not Iceberg.

Metrics: published, consumed, dead_letters, lag = published − consumed.
