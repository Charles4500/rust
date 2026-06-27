# Backend Authentication Service

A RESTful authentication service built with **Rust**, **Axum**, **Diesel**, and **PostgreSQL**. The application provides secure user authentication using email/password and Google OAuth 2.0. Passwords are hashed with Argon2, and authenticated sessions are managed using JWTs stored in HTTP-only cookies.

---

## Features

- User registration
- User login
- Google OAuth 2.0 authentication
- Password hashing using Argon2
- JWT authentication
- HTTP-only authentication cookies
- PostgreSQL database integration
- Diesel Async support
- Automatic database migrations on application startup

---

## Tech Stack

- Rust
- Axum
- Tokio
- PostgreSQL
- Diesel
- Diesel Async
- OAuth2
- Argon2
- JSON Web Tokens (JWT)
- Tower HTTP

---

## Prerequisites

Before running the application, ensure the following are installed:

- Rust (latest stable release)
- Cargo
- PostgreSQL
- Git

Verify the installation:

```bash
rustc --version
cargo --version
psql --version
```

---

## Clone the Repository

```bash
git clone https://github.com/your-username/backend.git
cd backend
```

Replace the repository URL with your own.

---

## Environment Variables

Create a `.env` file in the project root.

```env
DATABASE_URL=postgres://postgres:password@localhost:5432/auth_db

JWT_SECRET=your_super_secret_jwt_key

GOOGLE_CLIENT_ID=your_google_client_id
GOOGLE_CLIENT_SECRET=your_google_client_secret

REDIRECT_URL=http://localhost:3000/auth/google/callback

CLIENT_URL=http://localhost:3000
```

Replace all placeholder values with your own configuration.

---

## Database Setup

Create the PostgreSQL database.

```sql
CREATE DATABASE auth_db;
```

This project automatically executes database migrations when the application starts.

If you want to use the Diesel CLI instead, install it with:

```bash
cargo install diesel_cli --no-default-features --features postgres
```

Generate a migration:

```bash
diesel migration generate create_users
```

Run migrations manually:

```bash
diesel migration run
```

---

## Install Dependencies

Cargo automatically downloads all required dependencies.

```bash
cargo build
```

---

## Running the Application

Start the development server:

```bash
cargo run
```

The server starts on:

```text
http://localhost:8000
```

---

## Project Structure

```text
src/
├── common/
├── configs/
├── db/
├── dto/
├── handlers/
├── models/
├── routes/
├── schema/
├── state/
├── utils/
└── main.rs
```

---

## Available Endpoints

### Authentication

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/auth/register` | Register a new user |
| POST | `/auth/login` | Login with email and password |
| GET | `/auth/google` | Redirect to Google OAuth |
| GET | `/auth/google/callback` | Google OAuth callback |

---

## Development Commands

Build the project:

```bash
cargo build
```

Run the application:

```bash
cargo run
```

Run tests:

```bash
cargo test
```

Format the code:

```bash
cargo fmt
```

Run Clippy:

```bash
cargo clippy
```

Build an optimized release:

```bash
cargo build --release
```

---

## Authentication

After a successful login, the server issues a JWT that is stored as an HTTP-only cookie. This cookie is automatically included in subsequent authenticated requests by the browser.

---

## License

This project is provided for educational and development purposes. You are free to modify and extend it to suit your own requirements.