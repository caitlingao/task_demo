use std::env;
use std::ops::Deref;

use diesel::r2d2;
use rocket::{
    http::Status,
    request::{self, FromRequest},
    Outcome,
    Request,
    State,
};

use r2d2_redis::{
    RedisConnectionManager,
    redis::{self, Commands, RedisResult, FromRedisValue},
};
pub type Pool = r2d2::Pool<RedisConnectionManager>;

pub fn pool() -> Pool {
    let url   = env::var("REDIS_URL").unwrap();
    let manager = RedisConnectionManager::new(url).unwrap();
    let pool = r2d2::Pool::builder()
        .build(manager)
        .unwrap();

    pool
}

pub struct Conn(pub r2d2::PooledConnection<RedisConnectionManager>);

impl<'a, 'r> request::FromRequest<'a, 'r> for Conn {
    type Error = ();

    fn from_request(request: &'a request::Request<'r>) -> request::Outcome<Conn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Conn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for Conn {
    type Target = r2d2::PooledConnection<RedisConnectionManager>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn set_atomic_str_with_ttl(key: &str, value: &str, ttl_seconds: usize) -> RedisResult<()>{
    let pool = pool();
    let mut conn = pool.get().unwrap();
    conn.set(key, value)?;
    conn.expire(key, ttl_seconds)?;

    Ok(())
}

pub fn get_atomic_str(key: &str) -> RedisResult<String> {
    let pool = pool();
    let mut conn = pool.get().unwrap();
    let value = conn.get(key)?;
    FromRedisValue::from_redis_value(&value)
}


pub fn del_atomic_str(key: &str) -> RedisResult<()> {
    let pool = pool();
    let mut conn = pool.get().unwrap();
    conn.del(key)?;

    Ok(())
}
