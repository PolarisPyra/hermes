use axum::Router;
use sqlx::PgPool;

use super::{
    air::chunithm_air_routes, airplus::chunithm_airplus_routes, chunithm::chunithm_routes,
    plus::chunithm_plus_routes, verse::chunithm_verse_routes,
};

// Main router function that combines all version-specific routes
pub fn routes() -> Router<PgPool> {
    Router::new()
        .merge(chunithm_routes())
        .merge(chunithm_plus_routes())
        .merge(chunithm_air_routes())
        .merge(chunithm_airplus_routes())
        .merge(chunithm_verse_routes())
}
