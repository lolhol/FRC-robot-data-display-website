/// # Function
/// This is essentially a struct that can be parsed from an HTTP request. It contains the topic, amount, and time since the last update.
///
#[derive(serde::Serialize, serde::Deserialize)] // This code essentially says that this struct can be parsed from a json string.
pub struct Topic {
    pub topic: String,
    pub amount: Option<u32>, // option means that the value can be none or does not exist
    pub time_since_last_update: Option<u32>, // option means that the value can be none or does not exist
}
