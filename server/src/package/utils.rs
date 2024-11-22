use axum::extract::Multipart;
use tokio::{
    fs::{DirBuilder, File},
    io::AsyncWriteExt,
};
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
    match database::create_package(package).await {
        Ok(package_unchecked) => package_unchecked,
        Err(err_val) => {
            eprintln!("Error: Create Package | {}", err_val);
            None
        }
    }
}

pub async fn read_package(package_name: String) -> Option<Package> {
    match database::read_package(package_name).await {
        Ok(package_unchecked) => package_unchecked,
        Err(err_val) => {
            eprintln!("Error: Read Package | {}", err_val);
            None
        }
    }
}

pub async fn update_package(package_name: String, mut package: Package) -> Option<Package> {
    for dependency in package.get_dependencies() {
        if let Err(err_val) = database::read_package(dependency.to_string()).await {
            eprintln!("Error: Dependency | {}", err_val);
        }
    }
    package.set_last_update_date_time();
    match database::update_package(package_name, package).await {
        Ok(package_unchecked) => package_unchecked,
        Err(err_val) => {
            eprintln!("Error: Update Package | {}", err_val);
            None
        }
    }
}

pub async fn delete_package(package_name: String) -> Option<Package> {
    match database::delete_package(package_name).await {
        Ok(package_unchecked) => package_unchecked,
        Err(err_val) => {
            eprintln!("Error: Delete Package | {}", err_val);
            None
        }
    }
}

pub async fn download_package(package_name: String) -> Option<ReaderStream<File>> {
    let package = crate::package::utils::read_package(package_name).await?;
    let package_file_stream = match package.serve().await {
        Ok(package_file_stream) => package_file_stream,
        Err(err_val) => {
            eprintln!("Error: Download | File Stream | {}", err_val);
            return None;
        }
    };
    Some(package_file_stream)
}

pub async fn upload_package(mut package_file: Multipart) -> Option<Package> {
    let package_file_part = match package_file.next_field().await {
        Ok(field_unchecked) => field_unchecked?,
        Err(err_val) => {
            eprintln!("Error: Upload | Multipart | {}", err_val);
            return None;
        }
    };
    let package_file_name = package_file_part.name()?.to_string();

    let file_location = format!("{}/{}", PACKAGE_PATH, package_file_name);
    if let Err(_) = File::open(PACKAGE_PATH).await {
        if let Err(err_val) = DirBuilder::new().create(PACKAGE_PATH).await {
            eprintln!("Error: Upload | Create Directory | {}", err_val);
            return None;
        }
    }

    let package_file_data = match package_file_part.bytes().await {
        Ok(package_file_data) => package_file_data,
        Err(err_val) => {
            eprintln!("Error: Upload | Multipart Bytes | {}", err_val);
            return None;
        }
    };
    let mut package = crate::package::utils::read_package(package_file_name).await?;

    let mut file_descriptor = match File::create(&file_location).await {
        Ok(file_descriptor) => file_descriptor,
        Err(err_val) => {
            eprintln!(
                "Error: Upload | File Descriptor | {} |{}",
                file_location, err_val
            );
            return None;
        }
    };
    if let Err(err_val) = file_descriptor.write_all(&package_file_data).await {
        eprintln!("Error: Upload | File Descriptor Write | {}", err_val);
        return None;
    }

    package.set_location(&file_location.to_string());
    if let Err(err_val) = package.set_hash().await {
        eprintln!("Error: Hash | {}", err_val);
    }

    let package = crate::package::utils::update_package(package.get_name(), package).await?;
    Some(package)
}

pub async fn read_all_packages() -> Option<Vec<Package>> {
    match database::read_all_packages().await {
        Ok(package_unchecked) => {
            if package_unchecked.is_empty() {
                None
            } else {
                Some(package_unchecked)
            }
        }
        Err(err_val) => {
            eprintln!("Error: Read All Package | {}", err_val);
            None
        }
    }
}
