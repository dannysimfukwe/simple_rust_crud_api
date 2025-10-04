# 🦀 Simple Rust CRUD API

A minimal and modular Rust REST API built with **Axum** and **SQLx** for
PostgreSQL.\
This project provides a clear structure for creating and managing CRUD
operations efficiently.

------------------------------------------------------------------------

## ⚙️ Features

-   ✅ Modular route and model structure\
-   ✅ Full CRUD endpoints for `Item`\
-   ✅ PostgreSQL integration with SQLx\
-   ✅ JSON-based API responses\
-   ✅ Environment-based configuration via `.env`

------------------------------------------------------------------------

## 🚀 Getting Started

### 1. Clone and enter the project

``` bash
git clone https://github.com/dannysimfukwe/simple_rust_crud_api.git
cd simple_rust_crud_api
```

### 2. Set up environment variables

Create a `.env` file in the root directory:

``` bash
DATABASE_URL=postgres://user:password@localhost/rust_crud_db
APP_PORT=3000
```

### 3. Run database migrations

Make sure you have PostgreSQL running locally and run any necessary
migrations.

### 4. Run the server

``` bash
sqlx migrate run
cargo run
```

Server should start on:

    http://127.0.0.1:3000

------------------------------------------------------------------------

## 📂 Project Structure
    migrations
    src/
    ├── main.rs
    ├── db.rs
    ├── state.rs
    ├── models/
    │   └── item.rs
    ├── routes/
    │   ├── mod.rs
    │   └── items.rs

------------------------------------------------------------------------

## 📡 API Endpoints

  Method   Endpoint       Description
  -------- -------------- -------------------
  GET      `/items`       Fetch all items
  GET      `/items/:id`   Fetch single item
  POST     `/items`       Create new item
  PUT      `/items/:id`   Update item
  DELETE   `/items/:id`   Delete item

------------------------------------------------------------------------

## 🧠 Notes

-   Built with **Axum**, **Tokio**, and **SQLx**
-   Easy to expand with new routes and models
-   Designed for clean modular Rust development

------------------------------------------------------------------------

## 📝 License

This project is open-source and available under the [MIT
License](LICENSE).
