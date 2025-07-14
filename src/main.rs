use crate::server::Server;

mod handlers;
mod server;
mod router;

#[tokio::main]
async fn main() {
    Server::new("localhost:3000", "postgres://postgres:secret@localhost:5432/postgres")
        .await
        .start()
        .await;
}
