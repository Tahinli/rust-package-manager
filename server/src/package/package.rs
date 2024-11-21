use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_512};
use tokio::{fs::File, io::AsyncReadExt};
use tokio_util::io::ReaderStream;

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

impl Package {
    pub fn new(name: String, publisher: Publisher, version: Version) -> Self {
        Self {
            name,
            publisher,
            version,
            size: 0,
            hash: vec![],
            dependencies: vec![],
            publish_date_time: Utc::now(),
            last_update_date_time: Utc::now(),
            location: String::new(),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    pub fn get_location(&self) -> String {
        self.location.to_string()
    }

    pub async fn serve(&self) -> Option<ReaderStream<File>> {
        match File::open(self.get_location()).await {
            Ok(package_file) => Some(ReaderStream::new(package_file)),
            Err(_) => None,
        }
    }

    pub fn set_location(&mut self, location: &String) {
        self.location = location.to_owned()
    }

    pub fn get_dependencies(&self) -> &[String] {
        self.dependencies.as_slice()
    }

    pub fn set_last_update_date_time(&mut self) {
        self.last_update_date_time = Utc::now();
    }

    pub async fn set_hash(&mut self) {
        if let Ok(mut package_file) = File::open(self.get_location()).await {
            let mut hasher = Sha3_512::new();
            let mut data_buffer = vec![];
            if let Ok(_) = package_file.read_to_end(&mut data_buffer).await {
                hasher.update(data_buffer);
                self.hash = hasher.finalize().to_vec();
            }
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

impl Version {
    pub fn new(first: u8, second: u8, third: u8) -> Self {
        Version {
            first,
            second,
            third,
        }
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.first, self.second, self.third)
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

impl Publisher {
    pub fn new(name: String) -> Self {
        Publisher { name }
    }
}
