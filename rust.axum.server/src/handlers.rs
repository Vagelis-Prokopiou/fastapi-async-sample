use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use crate::AppState;
use crate::models::User;

pub async fn getUsers(State(state): State<AppState>) -> Result<Json<Vec<User>>, StatusCode> {
    let users = state.databaseService.getUsers().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    return Ok(Json(users));
}