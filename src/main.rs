use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use dotenvy::dotenv;
use rocket::{Error, State};
use std::env;

use self::schema::posts;

pub mod models;
pub mod schema;

#[macro_use]
extern crate rocket;

use rocket::serde::json::Value;
use rocket::serde::json::{json, Json};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

#[get("/")]
async fn index(_db: &State<PgPool>) -> Value {
    let mut conn = _db.get().unwrap();

    let posts = posts::table
        .select(post::as_select())
        .load(&mut conn)
        .expect("Error loading posts");

    json!(posts)
}

use self::models::{new_post, new_post_dto, post};

#[post("/add", data = "<post_request>")]
fn create_post(_db: &State<PgPool>, post_request: Json<new_post_dto<'_>>) -> Value {
    use crate::schema::posts;

    let mut conn = _db.get().unwrap();

    let new_post = new_post {
        title: &post_request.title,
        body: &post_request.body,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(post::as_returning())
        .get_result(&mut conn)
        .expect("Error saving new post");

    json!({ "status": "ok"})
}

// #[get("/delete")]
// fn delete() -> Value {
//     use self::schema::posts::dsl::*;

//     let connection = &mut establish_connection();

//     let result = posts
//         .limit(1)
//         .select(Post::as_select())

//         .load(connection)
//         .expect("Error loading posts");

//     diesel::delete(posts)
//         .filter(id.eq(result[0].id))
//         .returning(Post::as_returning())
//         .get_result(connection)
//         .expect("Error saving new post");

//     json!({ "status": "ok" })
// }

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let connection = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(connection)
        .expect("Error to build pool");

    rocket::build()
        .mount("/", routes![index, create_post])
        .manage(pool)
}
