use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager},
};

pub type Connection = PgConnection;

// pub type Pool = r2d2::Pool<ConnectionManager<Connection>>;