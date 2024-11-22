use rust_package_manager_client::user::user_interaction;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    user_interaction().await
}
