use axum::{Router, extract::State, http::StatusCode, routing::post};
use sqlx::PgPool;
use tracing::info;

// Handler specifically for AirPlus version of chunithm
pub async fn handle_game_event(_state: State<PgPool>) -> StatusCode {
    info!("ChunithmAirPlus GameEvent called");
    StatusCode::OK
}

// Handler specifically for AirPlus version of chunithm
pub async fn handle_settings(_state: State<PgPool>) -> StatusCode {
    info!("ChunithmAirPlus Settings called");
    StatusCode::OK
}

// Routes for Chunithm AirPlus version
pub fn chunithm_airplus_routes() -> Router<PgPool> {
    Router::new().nest(
        "/SDHD/115",
        Router::new().nest(
            "/ChuniServlet",
            Router::new()
                .route("/GameEvent", post(handle_game_event))
                .route("/Settings", post(handle_settings)),
        ),
    )
}
