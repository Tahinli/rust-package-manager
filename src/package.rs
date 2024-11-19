use std::fmt::Display;

use surrealdb::Datetime;

pub(crate) struct Package {
    name: String,
    publisher: Publisher,
    version: Version,
    size: u64,
    hash: String,
    publish_date_time: Datetime,
    last_update_date_time: Datetime,
    location: String,
}

impl Package {
    fn get_name(&self) -> String {
        self.name.to_string()
    }

    fn get_publisher_name(&self) -> String {
        self.publisher.get_name()
    }

    fn get_version(&self) -> String {
        self.version.to_string()
    }

    fn get_size(&self) -> u64 {
        self.size
    }

    fn get_hash(&self) -> String {
        self.hash.to_string()
    }

    fn get_publish_date_time(&self) -> String {
        self.publish_date_time.to_string()
    }

    fn get_last_update_date_time(&self) -> String {
        self.last_update_date_time.to_string()
    }

    fn get_location(&self) -> String {
        self.location.to_string()
    }
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
