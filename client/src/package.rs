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

impl Package {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_hash(&self) -> Vec<u8> {
        self.hash.to_vec()
    }
}

impl Default for Package {
    fn default() -> Self {
        Self {
            name: Default::default(),
            publisher: Default::default(),
            version: Default::default(),
            size: Default::default(),
            hash: Default::default(),
            dependencies: Default::default(),
            publish_date_time: Default::default(),
            last_update_date_time: Default::default(),
            location: Default::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    first: u8,
    second: u8,
    third: u8,
}

impl Default for Version {
    fn default() -> Self {
        Self {
            first: Default::default(),
            second: Default::default(),
            third: Default::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Publisher {
    name: String,
}

impl Default for Publisher {
    fn default() -> Self {
        Self {
            name: Default::default(),
        }
    }
}
