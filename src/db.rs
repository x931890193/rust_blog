use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

// let db_url = "postgres://username:password@localhost/rust_db"
pub fn create_db_pool(db_url: &str) -> Pool {
    // set up database connection pool
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    pool
}
