# 💬 Chat-App

**Chat-App** is a learning-focused Rust backend project designed to explore how to build a clean, scalable, and modern server-side application using Rust.  
The project serves as a hands-on workspace for practicing backend concepts such as API design, database migrations, containerization, and project structuring in Rust.

---

## 📌 Table of Contents

1. 📖 [Overview](#-overview)  
2. 🧰 [Tech Stack & Frameworks](#-tech-stack--frameworks)  
3. 🚀 [Getting Started](#-getting-started)  
4. 🧱 [Project Structure](#-project-structure)  

---

## 📖 Overview

**Chat-App** is a backend service written in Rust, intended for learning and experimentation.  
It focuses on:

- Clean backend architecture in Rust
- Modular and maintainable code organization
- Database integration and migrations
- Environment-based configuration
- Running services using Docker

This project is suitable for developers who want to learn how to build backend systems with Rust in a practical way.

---

### 🌐 Web Framework
- **Axum** *(or Actix-web — adjust if needed)*  
  - Async HTTP server
  - Routing and request handling
  - Middleware support

### ⚡ Async Runtime
- **Tokio**  
  - Asynchronous runtime for Rust
  - Powers async/await, networking, and concurrency

### 🗄️ Database
- **PostgreSQL** — Primary database
- **SQLx / SeaORM** *(adjust based on your project)*  
  - Async database access
  - Type-safe queries
  - Migration support

### 🔐 Authentication (Planned / Optional)
- **JWT (JSON Web Token)** for authentication and authorization

### 🐳 DevOps & Tooling
- **Docker** — Containerization
- **Docker Compose** — Local development environment
- **dotenv** — Environment variable management
- **Cargo** — Rust package manager & build tool

---

## 🚀 Getting Started

### 📦 Prerequisites

Make sure you have the following installed:

- Rust & Cargo (latest stable recommended)
- Docker & Docker Compose
- PostgreSQL (if running without Docker)

Verify installation:

```bash
rustc --version
cargo --version
```

## 🧱 Project Structure

```text
.
├── Cargo.toml              # Rust dependencies and project metadata
├── docker-compose.yaml     # Docker services configuration
├── src/                    # Application source code
├── entity/                 # Domain entities / models
├── migration/              # Database migration scripts
├── docs/                   # Documentation and learning notes
├── .env                    # Environment variables (ignored by git)
└── README.md