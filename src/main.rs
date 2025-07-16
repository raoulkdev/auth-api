use crate::server::Server;

mod handlers;
mod router;
mod server;
mod user;

#[tokio::main]
async fn main() {
    // Create and start a new server (Change localhost if you woult l and database)
    Server::new(
        "localhost:3000",
        "postgres://postgres:secret@localhost:5432/postgres",
    )
    .await
    .start()
    .await;
}
