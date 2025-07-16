# Rust Auth API

A simple user authentication API built with Rust using the Axum web framework and PostgreSQL. This project demonstrates safe password hashing, user creation, and login verification using modern Rust tools and practices.

## Features

- User registration with hashed passwords using Argon2
- Login endpoint to verify credentials
- PostgreSQL database with automatic table creation
- Secure error handling and basic validation
- Built using async Rust with Axum and SQLx

## Technologies

- [Axum](https://docs.rs/axum) - Web framework
- [Tokio](https://tokio.rs/) - Asynchronous runtime
- [SQLx](https://docs.rs/sqlx) - Async PostgreSQL support
- [Argon2](https://docs.rs/argon2) - Secure password hashing
- [UUID](https://docs.rs/uuid) - User ID generation
- [Chrono](https://docs.rs/chrono) - Date/time management

## Endpoints

### `POST /users`

Registers a new user.

**Body:**
```json
{
  "username": "your_username",
  "password": "your_password"
}
```

**Responses:**
- `201 Created`: User successfully created
- `400 Bad Request`: Username or password too short
- `409 Conflict`: Username already exists
- `500 Internal Server Error`: Failed to create user

---

### `POST /users/verify`

Verifies login credentials.

**Body:**
```json
{
  "username": "your_username",
  "password": "your_password"
}
```

**Responses:**
- `200 OK`: Verification successful
- `401 Unauthorized`: Invalid username or password
- `500 Internal Server Error`: Server error during verification

---

## Setup

### Prerequisites

- Rust (latest stable)
- PostgreSQL
- `sqlx-cli` (for migrations, optional)

### 1. Clone the repository

```bash
git clone https://github.com/your-username/rust-auth-api.git
cd rust-auth-api
```

### 2. Update your database connection

Edit `main.rs` if needed:

```rust
Server::new(
    "localhost:3000",
    "postgres://postgres:secret@localhost:5432/postgres"
)
```

Ensure the PostgreSQL database is running and accessible.

### 3. Run the server

```bash
cargo run
```

The API will be available at `http://localhost:3000`.

---

## Project Structure

```
src/
├── main.rs           # App entry point
├── server.rs         # Server setup and database config
├── router.rs         # Route definitions
├── handlers.rs       # Endpoint logic
└── user.rs           # Data models
```

---

## Database Schema

Created automatically if it does not exist:

```sql
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username TEXT UNIQUE NOT NULL,
    hashed_password TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    last_login_at TIMESTAMP WITH TIME ZONE NOT NULL
);
```

---

## License

This project is open-source and freely available for educational and development use.
