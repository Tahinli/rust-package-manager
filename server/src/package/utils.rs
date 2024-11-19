use std::path::PathBuf;

use axum::extract::Multipart;
use tokio::{fs::File, io::AsyncWriteExt};
use tokio_util::io::ReaderStream;

use crate::{database, routing, PACKAGE_PATH};

use super::package::{Package, Publisher, Version};

pub async fn create_package(package: routing::Package) -> Option<Package> {
    let publisher = Publisher::new(package.publisher);
    let version = package
        .version
        .split('.')
        .filter_map(|splitted| splitted.parse::<u8>().ok())
        .collect::<Vec<u8>>();
    let version = Version::new(*version.get(0)?, *version.get(1)?, *version.get(2)?);
    let package = Package::new(package.name, publisher, version);
    database::create_package(package).await
}

pub async fn read_package(package_name: String) -> Option<Package> {
    database::read_package(package_name).await
}

pub async fn update_package(package_name: String, mut package: Package) -> Option<Package> {
    for dependency in package.get_dependencies() {
        database::read_package(dependency.to_string()).await?;
    }
    package.set_last_update_date_time();
    database::update_package(package_name, package).await
}

pub async fn delete_package(package_name: String) -> Option<Package> {
    database::delete_package(package_name).await
}

pub async fn download_package(package_name: String) -> Option<ReaderStream<File>> {
    let package = crate::package::utils::read_package(package_name).await?;
    let package_file_stream = package.serve().await?;
    Some(package_file_stream)
}

pub async fn upload_package(mut package_file: Multipart) -> Option<Package> {
    let package_file_part = package_file.next_field().await.ok()??;
    let package_file_name = package_file_part.file_name()?.to_string();

    let file_location = format!("./{}/{}", PACKAGE_PATH, package_file_name);
    let file_location = PathBuf::from(file_location).canonicalize().ok()?;
    let file_location = file_location.to_str()?;

    let package_file_data = package_file_part.bytes().await.ok()?;
    let mut package = crate::package::utils::read_package(package_file_name).await?;

    let mut file_descriptor = File::create_new(&file_location).await.ok()?;
    file_descriptor.write_all(&package_file_data).await.ok()?;

    package.set_location(&file_location.to_string());
    package.set_hash().await;

    Some(package)
}

pub async fn read_all_packages() -> Option<Vec<Package>> {
    database::read_all_packages().await
}
