use axum::{Router, extract::State, http::StatusCode, routing::post};
use sqlx::PgPool;
use tracing::info;

// Handler specifically for Air version of chunithm
pub async fn handle_game_event(_state: State<PgPool>) -> StatusCode {
    info!("ChunithmAir GameEvent called");
    StatusCode::OK
}

// Handler specifically for Air version of chunithm
pub async fn handle_settings(_state: State<PgPool>) -> StatusCode {
    info!("ChunithmAir Settings called");
    StatusCode::OK
}

// Routes for Chunithm Air version
pub fn chunithm_air_routes() -> Router<PgPool> {
    Router::new().nest(
        "/SDHD/110",
        Router::new().nest(
            "/ChuniServlet",
            Router::new()
                .route("/GameEvent", post(handle_game_event))
                .route("/Settings", post(handle_settings)),
        ),
    )
}
