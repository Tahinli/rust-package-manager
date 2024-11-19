pub mod package;
pub mod utils;

pub struct PackageList {
    list: Vec<Package>,
}

impl PackageList {
    pub fn new() -> Self {
        PackageList { list: vec![] }
    }

    pub fn get_list(&self) -> Vec<&Package> {
        let mut list = vec![];
        self.list.iter().for_each(|package| list.push(package));
        list
    }

    pub fn insert(&mut self, package: Package) {
        self.list.push(package);
        self.sort();
    }

    fn sort(&mut self) {
        self.list.sort_by(|x,y|x.name.cmp(&y.name));
    }
}

pub struct Package {
    name: String,
    version: Version,
    location: String,
    size: usize,
    hash: String,
}

struct Version {
    first: usize,
    second: usize,
    third: usize,
}