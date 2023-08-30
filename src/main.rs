mod api;
mod discord;
mod models;
mod controller;

use controller::states::Control;

#[tokio::main]
async fn main() {
    // test
    // sleep(Duration::from_millis(1000)).await;
    let conn = Control::new();
    conn.run().await;
}
