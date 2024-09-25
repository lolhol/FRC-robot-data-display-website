#[derive(serde::Serialize, serde::Deserialize)]
pub struct Topic {
    pub topic: String,
    pub amount: Option<u32>,
    pub time_since_last_update: Option<u32>,
}
