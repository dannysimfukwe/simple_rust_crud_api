use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use crate::{
    models::item::{Item, CreateItem, UpdateItem},
    state::AppState,
};

// GET /items
pub async fn list_items(State(state): State<AppState>) -> Response {
    match sqlx::query_as::<_, Item>("SELECT id, name, description FROM items")
        .fetch_all(&state.db)
        .await
    {
        Ok(items) if items.is_empty() => (
            StatusCode::OK,
            Json(json!({ "message": "No items found", "data": [] })),
        )
            .into_response(),
        Ok(items) => (StatusCode::OK, Json(items)).into_response(),
        Err(e) => {
            eprintln!("DB error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Failed to fetch items" })),
            )
                .into_response()
        }
    }
}

// GET /items/:id
pub async fn get_item(Path(id): Path<i32>, State(state): State<AppState>) -> Response {
    match sqlx::query_as::<_, Item>("SELECT id, name, description FROM items WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(Some(item)) => (StatusCode::OK, Json(item)).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Item not found" })),
        )
            .into_response(),
        Err(e) => {
            eprintln!("DB error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Database query failed" })),
            )
                .into_response()
        }
    }
}

// POST /items
pub async fn create_item(State(state): State<AppState>, Json(item): Json<CreateItem>) -> Response {
    match sqlx::query_as::<_, Item>(
        "INSERT INTO items (name, description) VALUES ($1, $2) RETURNING id, name, description",
    )
    .bind(item.name)
    .bind(item.description)
    .fetch_one(&state.db)
    .await
    {
        Ok(created) => (StatusCode::CREATED, Json(created)).into_response(),
        Err(e) => {
            eprintln!("Insert failed: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Failed to create item" })),
            )
                .into_response()
        }
    }
}

// PATCH /items/:id/update
pub async fn update_item(
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Json(item): Json<UpdateItem>,
) -> Response {
    match sqlx::query_as::<_, Item>(
        "UPDATE items SET name = $1, description = $2 WHERE id = $3 RETURNING id, name, description",
    )
    .bind(item.name)
    .bind(item.description)
    .bind(id)
    .fetch_optional(&state.db)
    .await
    {
        Ok(Some(updated)) => (StatusCode::OK, Json(updated)).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Record not found" })),
        )
            .into_response(),
        Err(e) => {
            eprintln!("Update failed: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Database update failed" })),
            )
                .into_response()
        }
    }
}

// DELETE /items/:id/delete
pub async fn delete_item(Path(id): Path<i32>, State(state): State<AppState>) -> Response {
    match sqlx::query_as::<_, Item>(
        "DELETE FROM items WHERE id = $1 RETURNING id, name, description",
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await
    {
        Ok(Some(item)) => (StatusCode::OK, Json(item)).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Record not found" })),
        )
            .into_response(),
        Err(e) => {
            eprintln!("Delete failed: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Database error" })),
            )
                .into_response()
        }
    }
}
