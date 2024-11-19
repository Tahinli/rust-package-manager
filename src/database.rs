use std::time::Duration;

use surrealdb::{
    engine::remote::ws::{Client, Ws},
    Surreal,
};
use tokio::time::sleep;

pub async fn establish_connection() -> Surreal<Client> {
    Surreal::new::<Ws>("localhost:8000").await.unwrap()
}

pub async fn is_alive(db: Surreal<Client>) -> bool {
    tokio::select! {
        db_result = db.health() => { match db_result {
            Ok(_) => true,
            Err(_) => false,
        } },
        _ = sleep(Duration::from_secs(1)) => false
    }
}
