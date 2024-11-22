use tokio::{
    fs::{read_dir, remove_file, DirBuilder, File, OpenOptions},
    io::{AsyncReadExt, AsyncWriteExt},
};

use sha3::{Digest, Sha3_512};

const PACKAGE_PATH: &str = "./packages/";

pub async fn save_package(package_name: String, package_data: &[u8]) -> Result<(), std::io::Error> {
    let file_location = format!("{}{}", PACKAGE_PATH, package_name);
    if let Err(_) = File::open(PACKAGE_PATH).await {
        DirBuilder::new().create(PACKAGE_PATH).await?;
    }
    let mut package_file = File::create(file_location).await?;
    package_file.write_all(package_data).await?;
    save_metadata(package_name).await
}

pub async fn delete_package(package_name: String) -> Result<(), std::io::Error> {
    remove_file(format!("{}{}", PACKAGE_PATH, package_name)).await?;
    delete_metadata(package_name).await
}

pub async fn list_packages() -> Option<Vec<String>> {
    let mut folder_elements = read_dir(PACKAGE_PATH).await.ok()?;
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

async fn calculate_hash(package_name: String) -> Result<Option<Vec<u8>>, std::io::Error> {
    if let Some(_) = search_metadata(package_name.to_owned()).await? {
        let file_location = format!("{}{}", PACKAGE_PATH, package_name);
        let mut target_file = File::open(file_location).await?;
        let mut hasher = Sha3_512::new();
        let mut data_buffer = vec![];
        target_file.read_to_end(&mut data_buffer).await?;
        hasher.update(data_buffer);
        Ok(Some(hasher.finalize().to_vec()))
    } else {
        Ok(None)
    }
}

async fn search_metadata(package_name: String) -> Result<Option<usize>, std::io::Error> {
    let file_location = format!("{}{}", PACKAGE_PATH, "metadata.txt");
    let mut file = match File::open(file_location.clone()).await {
        Ok(file) => file,
        Err(_) => {
            return Ok(None);
        }
    };
    let mut file_data = String::default();
    _ = file.read_to_string(&mut file_data).await?;

    for (index, line) in file_data.lines().enumerate() {
        let line = line.trim_end();
        if line == package_name {
            return Ok(Some(index));
        }
    }
    Ok(None)
}

async fn search_and_retrieve_metadata(
    package_name: String,
) -> Result<(Option<usize>, Vec<String>), std::io::Error> {
    let file_location = format!("{}{}", PACKAGE_PATH, "metadata.txt");
    let mut file = match File::open(file_location.clone()).await {
        Ok(file) => file,
        Err(_) => {
            return Ok((None, vec![]));
        }
    };
    let mut file_data = String::default();
    _ = file.read_to_string(&mut file_data).await?;
    let mut lines = vec![];
    let mut target_index = None;
    for (index, line) in file_data.lines().enumerate() {
        if line.trim_end() == package_name {
            target_index = Some(index);
        }
        lines.push(line.to_string());
    }
    Ok((target_index, lines))
}

async fn save_metadata(package_name: String) -> Result<(), std::io::Error> {
    let searched = search_metadata(package_name.to_owned()).await?;
    if searched.is_none() {
        let file_location = format!("{}{}", PACKAGE_PATH, "metadata.txt");
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .write(true)
            .open(file_location)
            .await?;
        file.write_all(package_name.as_bytes()).await?;
        file.write_all(b"\n").await?;
    }
    Ok(())
}

async fn delete_metadata(package_name: String) -> Result<(), std::io::Error> {
    let (target_index, mut file_data) = search_and_retrieve_metadata(package_name).await?;
    let target_index = match target_index {
        Some(target_index) => target_index,
        None => return Err(std::io::ErrorKind::NotFound.into()),
    };

    let file_location = format!("{}{}", PACKAGE_PATH, "metadata.txt");
    let mut file = OpenOptions::new()
        .append(false)
        .create(false)
        .write(true)
        .open(file_location)
        .await?;
    file_data.remove(target_index);

    if file_data.is_empty() {
        file.set_len(0).await
    } else {
        file.write_all(&file_data.concat().as_bytes()).await
    }
}
