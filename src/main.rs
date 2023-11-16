use dotenv::dotenv;
use std::env;
use std::fmt;
use warp:: Filter;
use serde_json;
use pretty_env_logger;
use r2d2_sqlite::SqliteConnectionManager;
use r2d2::Pool;
use rusqlite::{params, Connection};

fn create_db_pool() -> Pool<SqliteConnectionManager> {
    let manager = SqliteConnectionManager::file("geo.db");
    Pool::new(manager).unwrap()
}

struct Street {
    name: String,
    lat:  f64,
    lon:  f64,
}
impl Street {
    fn new(name: &str, lat: f64, lon: f64) -> Self {
        Street {name: name.to_string(), lat, lon}
    }
}
impl fmt::Display for Street {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Name: {}, Latitude: {}, Longitude: {}",
               self.name, self.lat, self.lon)
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();

    let env_port = env::var("PORT")
        .unwrap()
        .parse()
        .expect("PORT environment variable must be a number");

    let my_street = Street::new("199 Testing Street", 40.15, -40.20);

    println!("Starting server on port {}...", env_port);
    println!("Testing Street model: {}", my_street);

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
