use network_tables::v4::MessageData;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct TableEntree {
    pub topic: String,
    pub value: String,
    pub timestamp: u32,
}

impl TableEntree {
    pub fn new(topic: String, value: String, timestamp: u32) -> Self {
        Self {
            topic,
            value,
            timestamp,
        }
    }

    pub fn from_message(data: MessageData) -> Self {
        //println!("{:?}", data);
        Self::new(data.topic_name, data.data.to_string(), data.timestamp)
    }

    pub fn get_error() -> Self {
        Self::new("ERROR".to_string(), "ERROR".to_string(), u32::MIN)
    }
}
