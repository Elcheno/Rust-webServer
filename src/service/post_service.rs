use crate::lib::connection::PgPool;

use crate::models::post_model::{NewPost, NewPostDto, Post};
use crate::models::response::{CustomError, NetworkResponse, Response, ResponseBody};

use rocket::serde::json::{json, Json, Value};
use rocket::State;

use crate::schema::posts;
use crate::schema::posts::dsl::*;

use diesel::prelude::*;

pub async fn get_posts(_db: &State<PgPool>) -> Result<Value, NetworkResponse> {
    let mut conn = _db.get().unwrap();

    let result = posts::table
        .select(Post::as_select())
        .load(&mut conn)
        .map_err(CustomError::from);

    match result {
        Ok(res) => Ok(json!(res)),
        Err(err) => Err(NetworkResponse::from(err)),
    }
}

pub async fn create_post(
    _db: &State<PgPool>,
    post_request: Json<NewPostDto<'_>>,
) -> Result<Value, NetworkResponse> {
    let mut conn = _db.get().unwrap();

    let new_post = NewPost {
        title: post_request.title,
        body: post_request.body,
    };

    let result: Result<Post, CustomError> = diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(&mut conn)
        .map_err(CustomError::from);

    match result {
        Ok(post) => Ok(json!(post)),
        Err(err) => Err(NetworkResponse::from(err)),
    }
}

pub async fn delete_post(_db: &State<PgPool>, uuid_dto: &str) -> Result<Value, NetworkResponse> {
    let mut conn = _db.get().map_err(|_| {
        // Crear un error de tipo Internal Server Error (500) para la gestion de errores como este.
        let response = Response {
            body: ResponseBody::Message(format!("Error connect with database")),
        };
        NetworkResponse::BadRequest(json!(response).to_string())
    })?;

    let uuid = uuid_dto.parse::<i32>().map_err(|_| {
        let response = Response {
            body: ResponseBody::Message(format!("Bad request - Invalid UUID format")),
        };
        NetworkResponse::BadRequest(json!(response).to_string())
    })?;

    let result = diesel::delete(posts)
        .filter(id.eq(uuid))
        .returning(Post::as_returning())
        .get_result(&mut conn)
        .map_err(CustomError::from);

    match result {
        Ok(post) => Ok(json!(post)),
        Err(err) => Err(NetworkResponse::from(err)),
    }
}
