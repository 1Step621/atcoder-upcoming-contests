use std::{convert::Infallible, env};

use axum::{http::StatusCode, response::IntoResponse, routing::get, Extension, Json, Router};
use dotenvy::dotenv;
use state::State;
use tokio::net::TcpListener;

mod periodic;
mod state;

async fn get_state(Extension(state): Extension<State>) -> Result<impl IntoResponse, Infallible> {
    let contests = state.contests.lock().unwrap().clone();
    Ok((StatusCode::OK, Json(contests)))
}

#[tokio::main]
async fn main() {
    dotenv().expect("Failed to load .env file");

    let state = State::default();

    tokio::spawn(periodic::wait(state.clone()));

    let app = Router::new()
        .route("/", get(get_state))
        .layer(Extension(state));

    let host = env::var("HOST").expect("Missing HOST environment variable");
    let port = env::var("PORT").expect("Missing PORT environment variable");
    let listener = TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
