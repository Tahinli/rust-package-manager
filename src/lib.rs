use surrealdb::{engine::remote::ws::Client, Surreal};

pub mod database;
pub mod http;
pub mod package;
pub mod routing;
pub mod utils;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db_client: Surreal<Client>,
}
