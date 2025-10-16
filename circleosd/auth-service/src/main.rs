mod registry;
mod service;
mod process;
mod health;

#[tokio::main]
async fn main() {
    println!("[service-registry] online");
    registry::run().await;
}
