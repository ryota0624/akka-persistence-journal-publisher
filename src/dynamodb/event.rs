use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DynamodbInsertEvent {
    #[serde(rename = "Records")]
    pub records: Vec<Record>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    #[serde(rename = "eventID")]
    pub event_id: String,
    pub event_version: String,
    pub dynamodb: Dynamodb,
    pub aws_region: String,
    pub event_source: String,
    pub event_name: String,
    #[serde(rename = "eventSourceARN")]
    pub event_source_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dynamodb {
    #[serde(rename = "ApproximateCreationDateTime")]
    pub approximate_creation_date_time: f64,
    #[serde(rename = "SizeBytes")]
    pub size_bytes: i64,
    #[serde(rename = "Keys")]
    pub keys: Keys,
    #[serde(rename = "NewImage")]
    pub new_image: NewImage,
    #[serde(rename = "StreamViewType")]
    pub stream_view_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Keys {
    pub pkey: Pkey,
    pub skey: Skey,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pkey {
    #[serde(rename = "S")]
    pub s: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Skey {
    #[serde(rename = "S")]
    pub s: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewImage {
    pub pkey: Pkey,
    pub ordering: Ordering,
    pub message: Message,
    #[serde(rename = "sequence-nr")]
    pub sequence_nr: SequenceNr,
    #[serde(rename = "persistence-id")]
    pub persistence_id: PersistenceId,
    pub skey: Skey,
    pub deleted: Deleted,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ordering {
    #[serde(rename = "N")]
    pub n: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    #[serde(rename = "B")]
    pub b: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SequenceNr {
    #[serde(rename = "N")]
    pub n: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersistenceId {
    #[serde(rename = "S")]
    pub s: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Deleted {
    #[serde(rename = "BOOL")]
    pub bool: bool,
}

