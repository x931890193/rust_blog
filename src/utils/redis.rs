extern crate r2d2;

pub use redis;

use lazy_static::lazy_static;

type RedisPool = r2d2::Pool<redis::Client>;


pub fn create_redis_pool(redis_url: &str) -> RedisPool {
    // set up redis connection pool
    let client: redis::Client = redis::Client::open(redis_url.to_string()).unwrap();
    // create r2d2 pool
    let pool: r2d2::Pool<redis::Client> = r2d2::Pool::builder().build(client).unwrap();
    pool
}

lazy_static! {
    pub static ref REDIS_POOL: RedisPool  = {
        let redis_url = "redis://127.0.0.1/1";
        let pool = create_redis_pool(&redis_url);
        pool
    };
}
