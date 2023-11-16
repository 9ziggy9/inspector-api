use dotenv::dotenv;
use std::env;
use warp::Filter;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let env_port = env::var("PORT")
        .unwrap()
        .parse()
        .expect("PORT environment variable must be a number");

    println!("Starting server on port {}...", env_port);

    let hello_world = warp::path!("hello" / "world")
        .map(|| warp::reply::html("<h1>Hello, World!</h1>"));

    warp::serve(hello_world)
        .run(([127, 0, 0, 1], env_port))
        .await;
}
