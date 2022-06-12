use std::{env, net::SocketAddr};

use axum::{routing::post, Extension, Router};
use sqlx::postgres::PgPoolOptions;

use crate::presentation::handle::{login_handle::login, signup_handle::signup};

mod application;
mod domain;
mod infrastructure;
mod presentation;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let uri = env::var("DATABASE_URL").unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&uri)
        .await
        .unwrap();

    let app = Router::new()
        // .route("/login", post(login))
        // .route("/signup", post(signup))
        .layer(Extension(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
