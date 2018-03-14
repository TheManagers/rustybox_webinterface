use std::error::Error;
use r2d2;
use r2d2_mongodb::{MongodbConnectionManager};
use iron::typemap::Key;

pub type MongodbPool = r2d2::Pool<MongodbConnectionManager>;
pub type MongodbConnection = r2d2::PooledConnection<MongodbConnectionManager>;

pub struct Mongodb;
impl Key for Mongodb {
    type Value = MongodbPool;
}

macro_rules! get_mongodb_connection {
    ($req:expr) => (match $req.get::<PRead<db::Mongodb>>() {
        Ok(pool) => match pool.get() {
            Ok(conn) => conn,
            Err(_) => {
                error!("Couldn't get a connection to Mongo!");
                return Ok(Response::with((status::InternalServerError)));
            }
        },
        Err(_) => {
            error!("Couldn't get the Mongo pool from the request!");
            return Ok(Response::with((status::InternalServerError)));
        }
    })
}

pub fn get_pool(host: &str, port: u16) -> Result<MongodbPool, Box<Error>> {
    let manager = MongodbConnectionManager::new(&host, port).unwrap();
    let pool = r2d2::Pool::builder().build(manager).unwrap();
    Ok(pool)
}