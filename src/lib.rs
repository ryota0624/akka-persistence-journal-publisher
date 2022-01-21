pub mod dynamodb;

pub mod proto {
    use prost::{DecodeError, Message};
    use crate::dynamodb::event::Dynamodb;
    use crate::proto::akka_persistence::PersistentMessage;
    use crate::proto::publisher_message::DeliveredDomainEvent;

    pub mod akka_persistence {
        include!(concat!(env!("OUT_DIR"), "/akka_persistence.rs"));
    }

    pub mod publisher_message {
        include!(concat!(env!("OUT_DIR"), "/publisher_message.rs"));
    }
    pub fn out_dir() {
        println!(env!("OUT_DIR"));
    }

    impl TryFrom<crate::dynamodb::event::Dynamodb> for PersistentMessage {
        type Error = DecodeError;

        fn try_from(dynamodb: Dynamodb) -> Result<Self, Self::Error> {
            PersistentMessage::decode(dynamodb.new_image.message.b.as_bytes())
        }
    }

    impl From<akka_persistence::PersistentMessage> for publisher_message::DeliveredDomainEvent {
        fn from(message: PersistentMessage) -> Self {
            DeliveredDomainEvent {
                body: message.payload.unwrap().payload,
                akka_persistence_sequence_nr: message.sequence_nr,
                akka_persistence_persistence_id: message.persistence_id,
                timestamp: message.timestamp.unwrap(),
            }
        }
    }

}
