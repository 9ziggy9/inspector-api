use dotenv::dotenv;
use std::env;
use std::fmt;
use warp:: Filter;
use serde::{Serialize, Deserialize};
use serde_json;
use pretty_env_logger;
use r2d2_sqlite::SqliteConnectionManager;
use r2d2::Pool;
use rusqlite::Result;

fn create_db_pool() -> Pool<SqliteConnectionManager> {
    let manager = SqliteConnectionManager::file("geo.db");
    Pool::new(manager).unwrap()
}

#[derive(Serialize, Deserialize)]
struct Street {
    id: i32,
    name: String,
    lat:  f64,
    lon:  f64,
}
impl Street {
    async fn fetch_all(pool: Pool<SqliteConnectionManager>) -> Result<Vec<Street>> {
        let conn = pool.get().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM streets;")?;
        let street_iter = stmt.query_map([], |row| {
            Ok(Street {
                id:   row.get(0)?,
                name: row.get(1)?,
                lat:  row.get(2)?,
                lon:  row.get(3)?,
            })
        })?;
        street_iter.collect()
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

    let db_pool = create_db_pool();

    let env_port = env::var("PORT")
        .unwrap()
        .parse()
        .expect("PORT environment variable must be a number");

    println!("Starting server on port {}...", env_port);

    // GET
    let get_hello = warp::path("hello")
        .map(|| warp::reply::html("<h1>Hello, World!</h1>"));
    let get_streets = warp::path("streets")
        .and_then(move || {
            let pool = db_pool.clone();
            async move {
                match Street::fetch_all(pool).await {
                    Ok(streets) => Ok(warp::reply::json(&streets)),
                    Err(_)      => Err(warp::reject::not_found()),
                }
            }
        });

    // POST
    let post_echo = warp::post()
        .and(warp::path("echo"))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .map(|body: serde_json::Value| warp::reply::json(&body));
    let post_street = warp::post()
        .and(warp::path("streets"))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .map(|body: serde_json::Value| warp::reply::json(&body));

    // Composing final set of routes
    let routes = get_hello
        .or(get_streets)
        .or(post_echo)
        .or(post_street)
        .with(warp::log("api"));

    warp::serve(routes)
        .run(([127, 0, 0, 1], env_port))
        .await;
}
