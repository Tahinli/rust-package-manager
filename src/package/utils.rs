use crate::{database, routing};

use super::package::Package;

pub async fn create_package(package: routing::Package) -> Option<Package> {
    let package = Package::new(package.name, package.publisher, package.version);
    database::create_package(package).await
}

pub async fn read_package(package_name: String) -> Option<Package> {
    database::read_package(package_name).await
}

pub async fn update_package(package_name: String, package: routing::Package) -> Option<Package> {
    let package = Package::new(package.name, package.publisher, package.version);
    database::update_package(package_name, package).await
}

pub async fn delete_package(package_name: String) -> Option<Package> {
    database::delete_package(package_name).await
}
