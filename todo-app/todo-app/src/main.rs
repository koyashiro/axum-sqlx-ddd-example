use std::{env, net::SocketAddr, sync::Arc};

use axum::{routing::post, Extension, Router};
use redis::Client;
use sqlx::postgres::PgPoolOptions;

use todo_app_application::usecase::{LoginUsecase, SignupUsecase};
use todo_app_infrastructure::{postgres::database::PgDB, redis::session::RedisSessionStore};
use todo_app_presentation::{
    handler::{login_handler::login, signup_handler::signup},
    session::SessionStore,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let uri = env::var("DATABASE_URL").unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&uri)
        .await
        .unwrap();

    let pg_db = Arc::new(PgDB::new(pool));
    let signup_usecase = SignupUsecase::new(pg_db.clone());
    let login_usecase = LoginUsecase::new(pg_db.clone());

    let redis_client = Client::open("redis://localhost/").unwrap();
    let session_store = Arc::new(RedisSessionStore::new(redis_client)) as Arc<dyn SessionStore>;

    let app = Router::new()
        .route("/login", post(login))
        .route("/signup", post(signup))
        .layer(Extension(pg_db))
        .layer(Extension(signup_usecase))
        .layer(Extension(login_usecase))
        .layer(Extension(session_store));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
