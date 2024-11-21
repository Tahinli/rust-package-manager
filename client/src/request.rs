use std::{error::Error, sync::LazyLock};

use crate::package::Package;

static CLIENT: LazyLock<reqwest::Client> = LazyLock::new(reqwest::Client::new);
const URL: &str = "http://localhost:2345";

pub async fn read_all_packages() -> Result<Vec<Package>, Box<dyn Error>> {
    Ok(CLIENT
        .get(format!("{}{}", URL, "/packages"))
        .send()
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await?
        .as_array()
        .map_or(Err(""), |values| Ok(values))?
        .iter()
        .map(|value| serde_json::from_value::<Package>(value.clone()).unwrap_or_default())
        .filter(|package| !package.get_name().is_empty())
        .collect())
}

pub async fn read_package(package_name: String) -> Option<Package> {
    CLIENT
        .get(format!("{}{}/{}", URL, "/packages", package_name))
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()?
}

pub async fn download_package(package_name: String) -> Option<Vec<u8>> {
    Some(
        CLIENT
            .get(format!("{}{}/{}", URL, "/packages/downloads", package_name))
            .send()
            .await
            .ok()?
            .bytes()
            .await
            .ok()?
            .to_vec(),
    )
}
