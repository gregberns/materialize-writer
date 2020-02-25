use crate::avro_encode::encode;
use avro_rs::schema::{RecordField, Schema, SchemaFingerprint, UnionSchema};
use avro_rs::types::Value;
use avro_rs::{from_value, types::Record, Codec, Reader, Writer};
use clap::{App, Arg};
use failure::bail;
use failure::Error;
use futures::StreamExt;
use futures_util::future::FutureExt;
use log::{info, warn};
use rdkafka::client::ClientContext;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::stream_consumer::{MessageStream, StreamConsumer};
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, Rebalance};
use rdkafka::error::KafkaResult;
use rdkafka::message::OwnedHeaders;
use rdkafka::message::{Headers, Message};
use rdkafka::producer::{BaseProducer, BaseRecord, DeliveryResult, ProducerContext};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::topic_partition_list::TopicPartitionList;
use rdkafka::util::get_rdkafka_version;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::future::Future;
use std::thread::sleep;
use std::time::Duration;

mod avro_encode;

struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    fn pre_rebalance(&self, rebalance: &Rebalance) {
        info!("Pre rebalance {:?}", rebalance);
    }

    fn post_rebalance(&self, rebalance: &Rebalance) {
        info!("Post rebalance {:?}", rebalance);
    }

    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        info!("Committing offsets: {:?}", result);
    }
}
type LoggingConsumer = StreamConsumer<CustomContext>;

#[tokio::main]
async fn main() {
    let matches = App::new("consumer example")
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or(""))
        .about("Simple command line consumer")
        .arg(
            Arg::with_name("brokers")
                .short("b")
                .long("brokers")
                .help("Broker list in kafka format")
                .takes_value(true)
                .default_value("localhost:9092"),
        )
        .arg(
            Arg::with_name("group-id")
                .short("g")
                .long("group-id")
                .help("Consumer group id")
                .takes_value(true)
                .default_value("example_consumer_group_id"),
        )
        .arg(
            Arg::with_name("log-conf")
                .long("log-conf")
                .help("Configure the logging format (example: 'rdkafka=trace')")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("topics")
                .short("t")
                .long("topics")
                .help("Topic list")
                .takes_value(true)
                .multiple(true)
                .required(true),
        )
        .get_matches();

    // setup_logger(true, matches.value_of("log-conf"));
    log4rs::init_file("log4rs.yml", Default::default())
        .expect("'log4rs.yml' not found. Required for logging.");

    // let (version_n, version_s) = get_rdkafka_version();
    // info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);

    let brokers = matches.value_of("brokers").unwrap();
    let topics = matches.values_of("topics").unwrap().collect::<Vec<&str>>();
    let group_id = matches.value_of("group-id").unwrap();
    info!("Brokers: ({:?})", brokers);
    info!("Topics: ({:?})", topics);
    info!("Group Id: ({:?})", group_id);

    let payload = serialize().unwrap();
    publish(brokers, topics[0], &payload).await;

    // let payload = serialize2().unwrap();
    // publish(brokers, topics[0], &payload).await;

    // let context = CustomContext;
    // let consumer = get_consumer(context, brokers, group_id, &topics);
    // process_message_stream(&consumer).await;
}

async fn publish(brokers: &str, topic_name: &str, payload: &[u8]) {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    let res = producer
        .send(
            FutureRecord::to(topic_name)
                .payload(payload)
                .key(&format!("Key1"))
                .headers(OwnedHeaders::new().add("header_key", "header_value")),
            0,
        )
        .await;

    info!("Future completed. Result: {:?}", res);
}

fn get_consumer<'a>(
    context: CustomContext,
    brokers: &str,
    group_id: &str,
    topics: &[&str],
) -> LoggingConsumer {
    let consumer: LoggingConsumer = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", brokers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        //.set("statistics.interval.ms", "30000")
        //.set("auto.offset.reset", "smallest")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(context)
        .expect("Consumer creation failed");

    consumer
        .subscribe(&topics.to_vec())
        .expect("Can't subscribe to specified topics");

    consumer
}

fn get_schema() -> Schema {
    let schema = r#"{
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
    }"#;
    // parse_schema(schema).unwrap()
    Schema::parse_str(schema).unwrap()
}

fn serialize() -> Result<Vec<u8>, Error> {
    let schema = get_schema();

    let mut writer = Writer::with_codec(&schema, Vec::new(), Codec::Null);

    let mut record = Value::Record(vec![
        (
            "before".to_string(),
            Value::Union(Box::new(Value::Record(vec![
                ("FirstName".to_string(), Value::String("Pink".to_string())),
                (
                    "LastName".to_string(),
                    Value::String("Elephants".to_string()),
                ),
            ]))),
        ),
        (
            "after".to_string(),
            Value::Union(Box::new(Value::Record(vec![
                (
                    "FirstName".to_string(),
                    Value::String("Donnatella".to_string()),
                ),
                ("LastName".to_string(), Value::String("Moss".to_string())),
            ]))),
        ),
    ]);

    writer.append(record)?;

    // let test = Test {
    //     a: 27,
    //     b: "foo".to_owned(),
    // };

    // writer.append_ser(test)?;

    writer.flush()?;

    let input = writer.into_inner();

    let body = [b'O', b'1', b'1', b'1', b'1'].to_vec();

    let output = [&body[..], &input[..]].concat();

    Ok(output)
}

fn serialize2() -> Result<Vec<u8>, Error> {
    let schema = get_schema();
    let mut writer = Writer::new(&schema, Vec::new());

    let record = Value::Record(vec![
        (
            "before".to_string(),
            Value::Union(Box::new(Value::Record(vec![
                ("FirstName".to_string(), Value::String("Greg".to_string())),
                ("LastName".to_string(), Value::String("Berns".to_string())),
            ]))),
        ),
        (
            "after".to_string(),
            Value::Union(Box::new(Value::Record(vec![
                (
                    "FirstName".to_string(),
                    Value::String("Hilbert".to_string()),
                ),
                ("LastName".to_string(), Value::String("McDugal".to_string())),
            ]))),
        ),
    ]);

    //add header info: '01111'
    let mut body = [b'O', b'1', b'1', b'1', b'1'].to_vec();
    encode(&record, &schema, &mut body);

    Ok(body)
}

fn deserialize(bytes: &Vec<u8>) -> Result<String, Error> {
    let schema = get_schema();

    let out = match avro_rs::Reader::with_schema(&schema, &bytes[..]) {
        Ok(reader) => {
            let value = reader.map(|v| format!("{:?}", v)).collect::<Vec<String>>();
            // let value =
            //     avro_rs::from_avro_datum(&schema, &mut bytes.clone(), Some(&schema));
            format!("Value: {:?}", value)
            // format!("{:?}", decode(&schema, &mut s.clone()))
        }
        Err(e) => {
            println!("Reader ERROR: {:?}", e);
            "".to_string()
        }
    };

    Ok(out)
}

#[test]
fn serialize_deserialize() {
    // let schema = get_schema();
    // println!("Schema: {:?}", schema);
    // panic!("forced panic");

    let bytes = serialize2().unwrap();
    println!(
        "in bytes: len {:?}, \n {:?}, \n {:?}",
        bytes.len(),
        bytes,
        std::string::String::from_utf8_lossy(&bytes.as_slice())
    );
    //This is failing by design right now
    let out = deserialize(&bytes);
    println!("Out: {:?}", out);
    panic!("forced panic");
}

#[derive(Debug)]
pub struct DiffPair {
    pub before: Option<avro_rs::types::Value>,
    pub after: Option<avro_rs::types::Value>,
}

async fn process_message_stream(consumer: &LoggingConsumer) {
    // let mut buffer = Vec::new();
    while true {
        if let Some(result) = consumer
            .get_base_consumer()
            .poll(std::time::Duration::from_millis(500))
        {
            match result {
                Ok(message) => {
                    println!("Message: {:?}", message);
                    // buffer.clear();
                    // buffer.extend_from_slice(message.payload().unwrap());
                }
                Err(err) => {
                    println!("Message error: {:?}", err);
                }
            }
        } else {
            println!("No message found");
        }
    }
}

fn decode(schema: &Schema, bytes: &mut &[u8]) -> Result<DiffPair, failure::Error> {
    let mut before = None;
    let mut after = None;
    let val = avro_rs::from_avro_datum(&schema, bytes, Some(&schema))?;
    match val {
        Value::Record(fields) => {
            for (name, val) in fields {
                if name == "before" {
                    before = Some(val); // extract_row(val, iter::once(Datum::Int64(-1)))?;
                } else if name == "after" {
                    after = Some(val); // extract_row(val, iter::once(Datum::Int64(1)))?;
                } else {
                    // Intentionally ignore other fields.
                }
            }
        }
        _ => bail!("avro envelope had unexpected type: {:?}", val),
    }
    Ok(DiffPair { before, after })
}
