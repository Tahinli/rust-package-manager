use std::{sync::LazyLock, time::Duration};

use surrealdb::{
    engine::remote::ws::{Client, Ws},
    Surreal,
};
use tokio::time::sleep;

static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

pub async fn establish_connection() -> Result<(), surrealdb::Error> {
    DB.connect::<Ws>("localhost:8000").await
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
