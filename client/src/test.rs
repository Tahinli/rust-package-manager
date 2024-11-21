#[cfg(test)]
use tokio::test;

#[test]
async fn test_read_all_packages() {
    use crate::request::read_all_packages;

    let packages = read_all_packages().await;

    assert_eq!(packages.is_ok(), true);
    assert_eq!(packages.as_ref().unwrap().is_empty(), false);
}

#[test]
async fn test_read_package() {
    use crate::request::read_package;

    let package = read_package("test_package".to_owned()).await;
    assert_eq!(package.is_some(), true);
    assert_eq!(package.unwrap().get_name().is_empty(), false);
}

#[test]
async fn test_download_package() {
    use crate::request::download_package;

    let package_data = download_package("test_package".to_owned()).await;
    assert_eq!(package_data.is_some(), true);
    assert_eq!(package_data.unwrap().is_empty(), false);
}

#[test]
async fn test_save_package() {
    use crate::request::download_package;

    let package_data = download_package("test_package".to_owned()).await;
    assert_eq!(package_data.is_some(), true);
    assert_eq!(package_data.as_ref().unwrap().is_empty(), false);

    use crate::file::save_package;

    let saved_or_not = save_package("test_package".to_string(), &package_data.unwrap()[..]).await;
    assert_eq!(saved_or_not.is_ok(), true);
}

#[test]
async fn test_delete_package() {
    use crate::request::download_package;

    let package_data = download_package("test_package".to_owned()).await;
    assert_eq!(package_data.is_some(), true);
    assert_eq!(package_data.as_ref().unwrap().is_empty(), false);

    use crate::file::save_package;

    let saved_or_not = save_package("test_package".to_string(), &package_data.unwrap()[..]).await;
    assert_eq!(saved_or_not.is_ok(), true);

    use crate::file::delete_package;

    let deleted_or_not = delete_package("test_package".to_string()).await;
    assert_eq!(deleted_or_not.is_ok(), true);
}
