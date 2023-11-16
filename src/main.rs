use dotenv::dotenv;
use std::env;
use warp::{http::StatusCode, Filter};
use serde::{Deserialize, Serialize};
use serde_json;
use pretty_env_logger;

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();

    let env_port = env::var("PORT")
        .unwrap()
        .parse()
        .expect("PORT environment variable must be a number");

    println!("Starting server on port {}...", env_port);

    // GET
    let route_hello = warp::path("hello")
        .map(|| warp::reply::html("<h1>Hello, World!</h1>"));

    // POST
    let route_echo = warp::post()
        .and(warp::path("echo"))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .map(|body: serde_json::Value| warp::reply::json(&body));

    let routes = route_hello
        .or(route_echo)
        .with(warp::log("api"));

    warp::serve(routes)
        .run(([127, 0, 0, 1], env_port))
        .await;
}
