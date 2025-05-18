use axum::{Router, extract::State, http::StatusCode, routing::post};
use sqlx::PgPool;
use tracing::info;

pub async fn handle_game_event(_state: State<PgPool>) -> StatusCode {
    info!("ChunithmPlus GameEvent called");
    StatusCode::OK
}

pub async fn handle_settings(_state: State<PgPool>) -> StatusCode {
    info!("ChunithmPlus Settings called");
    StatusCode::OK
}

// Routes for Chunithm Plus version
pub fn chunithm_plus_routes() -> Router<PgPool> {
    Router::new().nest(
        "/SDHD/105",
        Router::new().nest(
            "/ChuniServlet",
            Router::new()
                .route("/GameEvent", post(handle_game_event))
                .route("/Settings", post(handle_settings)),
        ),
    )
}
