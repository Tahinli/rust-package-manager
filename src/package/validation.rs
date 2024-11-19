use std::{fs::File, io::Read};

use crate::Package;

impl Package {
    pub(crate) fn calculate_hash(location: String) -> String {
        let mut file = File::open(location).unwrap();
        let mut buf = vec![];
        file.read_to_end(&mut buf).unwrap();
        String::new()
    }
}