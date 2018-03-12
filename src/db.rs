use std::error::Error;
use r2d2;
use r2d2_redis::{RedisConnectionManager};
use iron::typemap::Key;

pub type RedisPool = r2d2::Pool<RedisConnectionManager>;
pub type RedisConnection = r2d2::PooledConnection<RedisConnectionManager>;

pub struct Redis;
impl Key for Redis {
    type Value = RedisPool;
}

macro_rules! get_redis_connection {
    ($req:expr) => (match $req.get::<persistent::Read<db::PostgresDB>>() {
        Ok(pool) => match pool.get() {
            Ok(conn) => conn,
            Err(_) => {
                error!("Couldn't get a connection to redis!");
                return Ok(Response::with((status::InternalServerError)));
            }
        },
        Err(_) => {
            error!("Couldn't get the redis pool from the request!");
            return Ok(Response::with((status::InternalServerError)));
        }
    })
}

pub fn get_pool(uri: &str) -> Result<RedisPool, Box<Error>> {
    let manager = RedisConnectionManager::new(uri).unwrap();
    let pool = r2d2::Pool::builder().build(manager).unwrap();
    Ok(pool)
}