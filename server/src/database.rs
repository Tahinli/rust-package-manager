use std::{sync::LazyLock, time::Duration};

use surrealdb::{
    engine::remote::ws::{Client, Ws},
    Surreal,
};
use tokio::time::sleep;

use crate::package::package::Package;

static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

pub async fn establish_connection() -> Result<(), surrealdb::Error> {
    DB.connect::<Ws>("localhost:8000").await?;
    DB.use_ns("Packages").await?;
    DB.use_db("Packages").await?;
    DB.query("DEFINE TABLE Packages")
                .await.map(|_| ())?;
    DB.query("DEFINE INDEX package_nameINDEX on TABLE Packages COLUMNS package_name UNIQUE").await.map(|_| ())
                
    
}

pub async fn is_alive() -> bool {
    tokio::select! {
        db_result = DB.health() => { match db_result {
            Ok(_) => true,
            Err(_) => false,
        } },
        _ = sleep(Duration::from_secs(1)) => false
    }
}

pub async fn create_package(package: Package) -> Option<Package> {
    DB.create::<Option<Package>>(("Packages", package.get_name()))
        .content(package)
        .await
        .map_or_else(|_| None, |package| package)
}

pub async fn read_package(package_name: String) -> Option<Package> {
    DB.select(("Packages", package_name)).await.ok()?
}

pub async fn update_package(package_name: String, package: Package) -> Option<Package> {
    DB.update(("Packages", package_name))
        .content(package)
        .await.ok()?
}

pub async fn delete_package(package_name: String) -> Option<Package> {
    DB.delete(("Packages", package_name)).await.ok()?
}

pub async fn read_all_packages() -> Option<Vec<Package>> {
    DB.select::<Vec<Package>>("Packages").await.ok()
}
