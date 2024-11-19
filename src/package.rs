use crate::{Package, Version};

pub mod download;
pub mod install;
pub mod validation;

impl Package {
    pub fn new(name: String, version: String, location: String) -> Package {
        let version = Version::new(version).unwrap();
        let hash = Package::calculate_hash(location.to_string());
        Package {
            name,
            version,
            location,
            size: 0,
            hash, 
        }
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }
    pub fn get_version(&self) -> String {
        self.version.to_string()
    }
    pub fn get_location(&self) -> String {
        self.location.to_string()
    }
    pub fn get_size(&self) -> String {
        self.size.to_string()
    }
    pub fn get_hash(&self) -> String {
        self.hash.to_string()
    }
}


impl Version {
    fn new(version: String) -> Option<Self> {
        let splitted_input = version.split('.').collect::<Vec<&str>>();
        if splitted_input.len() == 3 {
            if let Ok(first) = splitted_input[0].parse::<usize>() {
                if let Ok(second) = splitted_input[1].parse::<usize>() {
                    if let Ok(third) = splitted_input[2].parse::<usize>() {
                        return Some(Version {
                            first,
                            second,
                            third,
                        });
                    }
                }
            }
        }
        None
    }
}

impl ToString for Version {
    fn to_string(&self) -> String {
        format!("{}.{}.{}", self.first, self.second, self.third)
    }
}
