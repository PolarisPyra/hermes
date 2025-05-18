mod controllers;

mod models;
mod titles {
    pub mod chunithm;
}
use crate::controllers::poweron::poweron;
use axum::{Router, extract::Extension, routing::post};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use titles::chunithm::index::routes;
use tokio::net::TcpListener;
use tracing::{Level, info};
use tracing_subscriber::{self};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();
    dotenv().ok();

    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new().connect(&url).await?;

    info!("Connected to the database");

    let allnet = Router::new()
        .route("/sys/servlet/PowerOn", post(poweron))
        .merge(routes().with_state(pool.clone()))
        .layer(Extension(pool.clone()));

    let aimedb = Router::new().layer(Extension(pool.clone()));

    let allnet_server = {
        let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
        info!("ALL.NET is running on http://127.0.0.1:8080");
        axum::serve(listener, allnet)
    };

    let aime_server = {
        let listener = TcpListener::bind("127.0.0.1:22345").await.unwrap();
        info!("AimeDB is running on http://127.0.0.1:22345");
        axum::serve(listener, aimedb)
    };

    tokio::select! {
        result = allnet_server => {
            if let Err(e) = result {
                tracing::error!("ALL.NET server error: {}", e);
            }
        },
        result = aime_server => {
            if let Err(e) = result {
                tracing::error!("AimeDB server error: {}", e);
            }
        },
    }

    Ok(())
}
