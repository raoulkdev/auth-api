use crate::server::Server;

mod handlers;
mod router;
mod server;
mod user;

#[tokio::main]
async fn main() {
    // Create and start a new server
    Server::new(
        "localhost:3000",                                           // Update this to wherever you want
        "postgres://postgres:secret@localhost:5432/postgres",   // Create a database and update this as well
    )
    .await
    .start()
    .await;
}
