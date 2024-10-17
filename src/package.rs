use std::fmt::Display;

use sha3::Sha3_512;
use surrealdb::Datetime;

pub(crate) struct Package {
    name: String,
    author: Publisher,
    version: Version,
    size: u64,
    publish_date_time: Datetime,
    last_update_date_time: Datetime,
    hash: Sha3_512,
    location: String,
}

pub(crate) struct Version {
    first: u8,
    second: u8,
    third: u8,
}

impl Version {
    fn new(first:u8, second:u8, third: u8) -> Self {
        Version {
            first,
            second,
            third,
        }
    }

    fn update(&mut self, first:u8, second:u8, third: u8) -> &Self {
        self.first = first;
        self.second = second;
        self.third = third;
        self
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.first, self.second, self.third)
    }
}

pub(crate) struct Publisher {
    name: String,
}

impl Publisher {
    fn new(name: String) -> Self {
        Publisher {
            name,
        }
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn update(&mut self, name: String) -> &Self {
        self.name = name;
        self
    }
}
