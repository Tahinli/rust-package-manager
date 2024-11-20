#[cfg(test)]
use tokio::test;

#[test]
async fn test_list_packages() {
    use crate::request::read_all_packages;

    let packages = read_all_packages().await;

    println!("{:#?}", packages);
    assert_eq!(packages.is_ok(), true);
}
