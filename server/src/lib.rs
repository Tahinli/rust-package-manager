pub mod database;
pub mod http;
pub mod package;
pub mod routing;

pub const PACKAGE_PATH: &str = "/packages";

#[derive(Debug, Clone)]
pub struct AppState {}
