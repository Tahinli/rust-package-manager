use surrealdb::{
    engine::remote::ws::{Client, Ws},
    Surreal,
};

pub async fn establish_connection() -> Surreal<Client> {
    Surreal::new::<Ws>("localhost:8000").await.unwrap()
}
