use std::path::PathBuf;

use axum::extract::Multipart;
use tokio::{fs::File, io::AsyncWriteExt};
use tokio_util::io::ReaderStream;

use crate::{database, routing, PACKAGE_PATH};

use super::package::Package;

pub async fn create_package(package: routing::Package) -> Option<Package> {
    let package = Package::new(package.name, package.publisher, package.version);
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
    if let Some(package) = crate::package::utils::read_package(package_name).await {
        if let Some(package_file_stream) = package.serve().await {
            return Some(package_file_stream);
        }
    }
    None
}

pub async fn upload_package(mut package_file: Multipart) -> Option<Package> {
    if let Ok(package_file_part_unchecked) = package_file.next_field().await {
        if let Some(package_file_part) = package_file_part_unchecked {
            if let Some(package_file_name) = package_file_part.file_name() {
                let package_file_name = package_file_name.to_owned();
                let file_location = format!("./{}/{}", PACKAGE_PATH, package_file_name);
                if let Ok(file_location) = PathBuf::from(file_location).canonicalize() {
                    if let Some(file_location) = file_location.to_str() {
                        if let Ok(package_file_data) = package_file_part.bytes().await {
                            if let Some(mut package) =
                                crate::package::utils::read_package(package_file_name.to_owned())
                                    .await
                            {
                                if let Ok(mut file_descriptor) =
                                    File::create_new(&file_location).await
                                {
                                    if let Ok(_) =
                                        file_descriptor.write_all(&package_file_data).await
                                    {
                                        package.set_location(&file_location.to_string());
                                        package.set_hash().await;
                                        return Some(package);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    None
}
