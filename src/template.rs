pub fn get_env_template(name: &str) -> String {
    format!(
        r#"# Database Configuration
DB_HOST=127.0.0.1
DB_PORT=5432
DB_USER=postgres
DB_PASSWORD=postgres
DB_NAME={name}

# Api Configuration
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
API_VERSION=v1
LOG_LEVEL=debug

# Web Configuration
APP_HOST=127.0.0.1
APP_PORT=3000
NODE_ENV=development"#,
        name = name,
    )
}

pub fn get_gitignore_template() -> String {
    r#"# Dependencies
/node_modules
/target
/dist
/build
/.pnp
.pnp.js
cargo.lock

# Testing
/coverage

# Production
/build

# Environment Variables
.env
.env.*
!.env.example

# IDE - VSCode
.vscode/*
!.vscode/settings.json
!.vscode/tasks.json
!.vscode/launch.json
!.vscode/extensions.json

# IDE - IntelliJ
.idea/
*.iml
*.iws

# IDE - Vim
*.swp
*.swo

# Logs
logs
*.log
npm-debug.log*
yarn-debug.log*
yarn-error.log*

# OS
.DS_Store
.DS_Store?
._*
.Spotlight-V100
.Trashes
ehthumbs.db
Thumbs.db

# Database
*.sqlite
*.sqlite3
*.db
/database

# Rust specific
debug/
target/
**/*.rs.bk
Cargo.lock

# Generated documentation
/doc"#
        .to_string()
}

pub fn get_nebula_template(
    name: &str,
    project_type: &str,
    database: &str,
    server_type: &str,
) -> String {
    format!(
        r#"[project]
name = "{name}"
type = "{project_type}"  # web, api, full
database = "{database}"  # mysql, postgresql, mariadb, mongodb
server_type = "{server_type}"  # rest, graphql

[paths]
src = "src"
api = "src/api"
web = "src/web"
migrations = "migrations"
templates = "templates"
static = "static"
tests = "tests"

[environment]
# Database Configuration
DB_HOST = "127.0.0.1"
DB_PORT = "5432"
DB_USER = "postgres"
DB_PASSWORD = "postgres"
DB_NAME = "{name}"

# Backend Configuration
SERVER_HOST = "127.0.0.1"
SERVER_PORT = "8080"
API_VERSION = "v1"
LOG_LEVEL = "debug"

# Frontend Configuration
APP_HOST = "127.0.0.1"
APP_PORT = "3000"
NODE_ENV = "development"

[deployment]
provider = "docker"
container_name = "{name}"
docker_image = "rust:latest"
docker_compose = true

[security]
jwt_enabled = true
cors_enabled = true
rate_limit = true
allowed_origins = ["http://localhost:3000"]"#,
        name = name,
        project_type = project_type,
        database = database,
        server_type = server_type
    )
}

pub fn get_readme_template(name: &str) -> String {
    format!(
        r#"# {name}

## Prerequisites
- Rust (latest stable)
- PostgreSQL
- Node.js (if using frontend)

## Quick Start

### Installation
```bash
# Clone the repository
git clone <repository-url>
cd {name}

# Install dependencies
cargo build

# Set up environment variables
cp .env.example .env

# Run migrations
cargo run -- db migrate

# Start the server
cargo run
"#,
        name = name,
    )
}

pub fn get_cargo_template(name: &str) -> String {
    format!(
        r#"[package]
name = "{name}"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = {{ version = "1.0", features = ["full"] }}
axum = {{ version = "0.8.0", features = ["macros"] }}
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0"
tower = "0.5.2"
tower-http = {{ version = "0.6.2", features = ["cors"] }}
dotenvy = "0.15"
tracing = "0.1"
tracing-subscriber = {{ version = "0.3", features = ["env-filter"] }}
thiserror = "2.0.10"
chrono = {{ version = "0.4", features = ["serde"] }}
validator = {{ version = "0.20.0", features = ["derive"] }}
async-graphql = {{ version = "5.0.7", features = ["chrono"] }}"#,
        name = name
    )
}

pub fn get_entity_template(name: &str, fields: &str) -> String {
    format!(
        r#"use serde::{{Deserialize, Serialize}};
use async_graphql::{{SimpleObject, InputObject}};
use validator::Validate;
use chrono::{{DateTime, Utc}};

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, Validate)]
pub struct {name} {{
    #[serde(default)]
    pub id: i32,
    {fields},
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}}

#[derive(InputObject, Validate)]
pub struct {name}Input {{
    {fields}
}}

#[derive(InputObject)]
pub struct Update{name}Input {{
    {fields}
}}"#,
        name = name,
        fields = fields
    )
}

pub fn get_main_template() -> String {
    r#"mod server;
mod route;
mod middleware;

use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    // Load environment variables
    dotenv().ok();

    // Start server
    server::Server::run().await?;
    
    Ok(())
}
"#
    .to_string()
}

pub fn get_server_template() -> String {
    r#"use std::env;
use std::net::SocketAddr;
use axum::{routing::get, Router};
use tokio::net::TcpListener;
use crate::route;
use crate::middleware;

#[derive(Debug)]
pub struct Server;

impl Server {
    pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
        // Load environment variables
        let host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port: u16 = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .expect("SERVER_PORT must be a valid integer");

        // Configure server address
        let addr: SocketAddr = format!("{}:{}", host, port)
            .parse()
            .expect("Failed to parse socket address");

        // Setup routes and middleware
        let app = Router::new()
            .merge(route::configure())
            .layer(middleware::cors());

        // Create and bind TCP listener
        let listener = TcpListener::bind(addr).await?;
            
        println!("🚀 Server started on http://{}", addr);
        
        // Start server
        axum::serve(listener, app).await?;
        Ok(())
    }
}
"#
    .to_string()
}

pub fn get_route_template() -> String {
    r#"use axum::{
    routing::{get, post},
    Router,
    response::Json,
};
use serde_json::json;

pub fn configure() -> Router {
    Router::new()
        .route("/", get(health_check))
        .route("/api/v1/hello", get(hello_world))
}

async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION"),
        "name": env!("CARGO_PKG_NAME")
    }))
}

async fn hello_world() -> Json<serde_json::Value> {
    Json(json!({
        "message": "Hello, World!"
    }))
}
"#
    .to_string()
}
