use std::env;

use axum::{extract::Request, http::StatusCode, response::IntoResponse, routing::post, Router};

mod remote;

async fn handler(req: Request) -> impl IntoResponse {
    if !req
        .headers()
        .get("Authorization")
        .is_some_and(|x| x.to_str().unwrap() == env::var("TOKEN").unwrap())
    {
        return (StatusCode::FORBIDDEN, "");
    }

    remote::trigger();

    println!("Gate opened");

    (StatusCode::OK, "OK")
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", post(handler));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
