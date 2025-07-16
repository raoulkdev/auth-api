use crate::router::router;
use colored::Colorize;
use sqlx::{PgPool, Pool, Postgres};
use std::sync::Arc;
use tokio::net::TcpListener;

// Server struct & implementation
pub struct Server {
    address: &'static str,
    users_database: Arc<Pool<Postgres>>,
}
impl Server {
    // Create a new server instance
    pub async fn new(address: &'static str, database_url: &'static str) -> Self {
        Self {
            address,
            users_database: Arc::new(
                Self::setup_database(database_url)
                    .await
                    .expect("Could not connect connect to database!"),
            ),
        }
    }

    // Setup & connect to the database
    pub async fn setup_database(database_url: &'static str) -> Result<Pool<Postgres>, String> {
        let pool = PgPool::connect(database_url).await;
        match pool {
            Ok(db) => {
                println!(
                    "{}{database_url}",
                    "Successfully connected to database at: ".bright_green()
                );
                sqlx::query(
                    "
                CREATE TABLE IF NOT EXISTS users (
                    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                    username TEXT UNIQUE NOT NULL,
                    hashed_password TEXT NOT NULL,
                    created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
                    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
                    last_login_at TIMESTAMP WITH TIME ZONE NOT NULL
                )",
                )
                .execute(&db)
                .await
                .unwrap();
                Ok(db)
            }
            Err(err) => Err(format!("{err}")),
        }
    }

    // Start the server
    pub async fn start(&self) {
        let listener = TcpListener::bind(self.address).await;
        match listener {
            Ok(l) => {
                println!(
                    "{}{}",
                    "Successfully started server at: ".bright_green(),
                    l.local_addr().unwrap()
                );
                axum::serve(l, router(self.users_database.clone()))
                    .await
                    .unwrap()
            }
            Err(e) => {
                println!("{}{e}", "Could not start server: ".bright_red());
            }
        }
    }
}
