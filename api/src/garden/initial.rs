use axum::{extract::Json, response::Redirect, routing::get, routing::post, Router};

use super::types::*;

async fn handle_post(Json(mut Fella): Json<student::Student>) -> Json<student::Student> {
    println!("{Fella:?}");
    Fella.age += 15;
    Json::from(Fella)
}

pub fn get_initial_routes() -> Router {
    let initial = Router::new()
        .route("/health", get(|| async { "initial running" }))
        .route("/poste_ai", post(handle_post));

    initial
}
