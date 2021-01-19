use std::env;
use std::ops::Deref;

use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager},
};

use rocket::{http::Status, request::{self, FromRequest}, Outcome, Request, State, Rocket};

embed_migrations!();

pub type Connection = PgConnection;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn pool() -> Pool {
    let database_url = env::var("DATABASE_URL").expect("set DATABaSe");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");

    pool
}

pub fn run_db_migrations(rocket: Rocket) -> Result<Rocket, Rocket> {
    let pool = pool();
    match embedded_migrations::run(&pool.get().unwrap()) {
        Ok(()) => Ok(rocket),
        Err(err) => {
            error!("failed to run database migrations: {:?}", err);
            Err(rocket)
        }
    }
}

pub struct Conn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for Conn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Conn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Conn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for Conn {
    type Target = PgConnection;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
