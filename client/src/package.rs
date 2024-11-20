use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    name: String,
    publisher: Publisher,
    version: Version,
    size: u64,
    hash: Vec<u8>,
    dependencies: Vec<String>,
    publish_date_time: DateTime<Utc>,
    last_update_date_time: DateTime<Utc>,
    location: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    first: u8,
    second: u8,
    third: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Publisher {
    name: String,
}
