# Materialize Publish Client POC

This is an attempt to pass data to Materialize via the current Debizum Avro/Kafka mechanism, but several different workflow attempts have failed.

See [#2125](https://github.com/MaterializeInc/materialize/issues/2125) for some more detail.

## Errors

In the example code, there are two attempts at passing a message:

### Kafka Message with Schema

`src/main.rs:serialize()` will use the [`MaterializeInc/avro_rs`](https://github.com/MaterializeInc/avro-rs/) version of the Avro `Writer`.

When Materialized processes this message it fails with the error at the [top of `decode`](https://github.com/MaterializeInc/materialize/blob/master/src/interchange/avro.rs#L394).

When I looked at the message in Kafka, the Avro `Writer` is putting the whole schema in the payload.

### Kafka Message with Without Schema

So instead of using the `Writer` directly I wanted to encode the `Value::Record` and send it, as seen in `src/main.rs:serialize2()`.

To do this, I extracted the private [MaterializeInc/avro_rs `encode()` method](https://github.com/MaterializeInc/avro-rs/blob/master/src/encode.rs) into `src/avro_encode.rs` to test it out.

When I run the workflow below, I get all the way to the last step which fails with `column "FirstName" does not exist`, which you can find in the Materialized source code as `column \"{}\" does not exist`.

## Workflow

1) Build the things

```
docker-compose build
```

1) Start Kafka + Zookeeper

```
docker-compose up -d kafka
```

1) Write messages to Kafka

```
docker-compose up writer
```

1) Check messages were written

```
docker-compose run inspect bash
kafkacat -L -b kafka:9092 -t topic1
kafkacat -C -b kafka:9092 -t topic1
```

1) Start Materialized

```
docker-compose up materialized
```

1) Start Materialize CLI

```
docker-compose up cli
```

1) Define Source

```
CREATE SOURCE topic1
FROM KAFKA BROKER 'kafka:9092' TOPIC 'topic1'
FORMAT AVRO USING SCHEMA '{
        "type": "record",
        "name": "envelope",
        "fields": [
            {
                "name": "before",
                "type": [
                    "null",
                    {
                        "type": "record",
                        "name": "row",
                        "fields": [
                            {"name": "FirstName", "type": "string"},
                            {"name": "LastName", "type": "string"}
                        ]
                    }
                ]
            },
            {
                "name": "after",
                "type": [
                    "null",
                    {
                        "type": "record",
                        "name": "row",
                        "fields": [
                            {"name": "FirstName", "type": "string"},
                            {"name": "LastName", "type": "string"}
                        ]
                    }
                ]
            }
        ]
    }'
ENVELOPE DEBEZIUM;
```

1) Add View

```
CREATE MATERIALIZED VIEW topic1_view AS
SELECT FirstName, LastName FROM topic1
```
