use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use std::env;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> ConnectionManager<PgConnection> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = ConnectionManager::<PgConnection>::new(database_url);
    connection
}

pub fn build_pool() -> Pool<ConnectionManager<PgConnection>> {
    let connection = establish_connection();
    let pool = Pool::builder()
        .build(connection)
        .expect("Error to build pool");
    pool
}
