use axum::{response::Redirect, routing::get, Router};

use crate::garden::initial::get_initial_routes;

pub mod garden;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .fallback(Redirect::to("/"))
        .nest("/initial", get_initial_routes());

    // app.route(path, method_router)

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
