use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use rocket::serde::json::json;
use rocket::State;
use std::env;

use crate::models::response::{NetworkResponse, Response, ResponseBody};

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

pub fn get_conn(
    _db: &State<PgPool>,
) -> Result<PooledConnection<ConnectionManager<PgConnection>>, NetworkResponse> {
    let response = _db.get().map_err(|_| {
        let response = Response {
            body: ResponseBody::Message(format!(
                "Internal Server Error - Error connect with database"
            )),
        };
        NetworkResponse::InternalServerError(json!(response).to_string())
    });

    response
}
