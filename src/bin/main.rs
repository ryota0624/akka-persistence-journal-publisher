use std::env;
use std::str::FromStr;

use aws_sdk_sns;
use aws_sdk_sns::Endpoint;
use aws_types::region::Region;
use lambda_runtime::{Context, Error, handler_fn};
use prost::Message;
use serde_json::{json, Value};

use akka_persistence_journal_publisher::dynamodb::event::DynamodbInsertEvent;
use akka_persistence_journal_publisher::proto::akka_persistence::PersistentMessage;
use akka_persistence_journal_publisher::proto::publisher_message::DeliveredDomainEvent;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler = handler_fn(handle_event);
    lambda_runtime::run(handler).await?;
    Ok(())
}

async fn handle_event(event: Value, _: Context) -> Result<Value, Error> {
    let aws_region = env::var("AWS_DEFAULT_REGION").expect("require AWS_DEFAULT_REGION in env");
    let credentials_provider =
        aws_config::default_provider::credentials::default_provider().await;

    let sns_client = aws_sdk_sns::client::Client::from_conf(
        aws_sdk_sns::Config::builder().region(
            Region::new(aws_region)
        ).credentials_provider(credentials_provider).endpoint_resolver(
            Endpoint::immutable(http::Uri::from_str(
                &env::var("SNS_ENDPOINT").expect("require SNS_ENDPOINT in env"),
            ).unwrap())
        )
            .build()
    );

    let event: DynamodbInsertEvent = serde_json::from_value(event)?;

    log::info!("record counts: {}", event.records.len());

    let domain_events = event.records.into_iter().map(|record| {
        DeliveredDomainEvent::from(
            PersistentMessage::try_from(record.dynamodb)
                .expect("dynamodb record should be decodable to PersistentMessage")
        )
    });


    let publish_batch_request_entries = domain_events.into_iter().map(|message| {
        aws_sdk_sns::model::PublishBatchRequestEntry::builder()
            .id(message.akka_persistence_persistence_id.clone().expect("persistence_id should be not None").to_string())
            .message(base64::encode(message.encode_to_vec())).build()
    });

    let mut publish = sns_client.publish_batch();
    for entry in publish_batch_request_entries {
        publish = publish.publish_batch_request_entries(entry);
    }

    publish.send().await
        .map(|_| json!({ "message": "publishing succeeded" }))
        .map_err(|err| {
            Error::try_from(anyhow::Error::msg(err.to_string())).unwrap()
        })
}
