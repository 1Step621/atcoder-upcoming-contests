use std::convert::Infallible;

use axum::{http::StatusCode, response::IntoResponse, routing::get, Extension, Json, Router};
use state::State;

mod periodic;
mod state;

async fn get_state(Extension(state): Extension<State>) -> Result<impl IntoResponse, Infallible> {
    let contests = state.contests.lock().unwrap().clone();
    Ok((StatusCode::OK, Json(contests)))
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let state = State::default();

    tokio::spawn(periodic::wait(state.clone()));

    let router = Router::new()
        .route("/", get(get_state))
        .layer(Extension(state));
    Ok(router.into())
}
