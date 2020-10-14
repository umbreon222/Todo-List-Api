use std::env;
use dotenv::dotenv;
use diesel::sqlite::SqliteConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;

pub type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

pub fn get_pool() -> SqlitePool {
    // TODO: pass the connection URL into this function rather than extracting
    // it from the environment within this function
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("no DB URL"); // TODO: handle errors
    let mgr = ConnectionManager::<SqliteConnection>::new(url);
    r2d2::Pool::builder()
        .build(mgr)
        .expect("Failed to create connection pool") // TODO: handle errors
}
