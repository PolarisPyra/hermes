use axum::{Router, extract::State, http::StatusCode, routing::post};
use sqlx::PgPool;
use tracing::info;

// Handler specifically for base version of chunithm
pub async fn handle_game_event(_state: State<PgPool>) -> StatusCode {
    info!("Chunithm Base GameEvent called");
    StatusCode::OK
}

// Handler specifically for base version of chunithm
pub async fn handle_settings(_state: State<PgPool>) -> StatusCode {
    info!("Chunithm Base Settings called");
    StatusCode::OK
}

// Routes for original Chunithm version
pub fn chunithm_routes() -> Router<PgPool> {
    Router::new().nest(
        "/SDHD/100", // Version path for original Chunithm
        Router::new().nest(
            "/ChuniServlet",
            Router::new()
                .route("/GameEvent", post(handle_game_event))
                .route("/Settings", post(handle_settings)),
        ),
    )
}
