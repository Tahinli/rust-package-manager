use tokio::{
    fs::{read_dir, remove_file, File},
    io::AsyncWriteExt,
};

const FILE_LOCATION: &str = "./packages/";

pub async fn save_package(package_name: String, package_data: &[u8]) -> Result<(), std::io::Error> {
    let file_location = format!("{}{}", FILE_LOCATION, package_name);
    let mut package_file = File::create(file_location).await?;
    package_file.write_all(package_data).await
}

pub async fn delete_package(package_name: String) -> Result<(), std::io::Error> {
    remove_file(format!("{}{}", FILE_LOCATION, package_name)).await
}

pub async fn list_packages() -> Option<Vec<String>> {
    let mut folder_elements = read_dir(FILE_LOCATION).await.ok()?;
    let mut packages = vec![];
    loop {
        match folder_elements.next_entry().await.ok()? {
            Some(file_entry) => packages.push(file_entry.file_name().into_string().ok()?),
            None => break,
        }
    }

    if packages.is_empty() {
        return None;
    }
    Some(packages)
}
