use diesel::prelude::*;

use rocket::serde::json::Value;
use rocket::serde::json::{json, Json};
use rocket::State;

use crate::models::jwt_model::JWT;
use crate::models::post_model::{NewPostDto, Post, UpdatePostDto};
use crate::models::response::NetworkResponse;

use crate::schema::posts;
use crate::schema::posts::dsl::*;

use crate::lib::connection::PgPool;
use crate::service::post_service;

#[get("/post")]
pub async fn get_post(_db: &State<PgPool>, key: Result<JWT, NetworkResponse>,) -> Result<Value, NetworkResponse> {
    let _jwt = key?;

    let response = post_service::get_posts(_db).await;

    match response {
      Ok(value) => Ok(value),
      Err(err) => Err(err)
    }
}

#[post("/post", data = "<post_request>")]
pub async fn create_post(
    _db: &State<PgPool>,
    post_request: Json<NewPostDto<'_>>,
    key: Result<JWT, NetworkResponse>,
) -> Result<Value, NetworkResponse> {
    let _jwt = key?;

    let response = post_service::create_post(_db, post_request).await;

    match response {
        Ok(value) => Ok(value),
        Err(err) => Err(err),
    }
}

#[delete("/post/<uuid_dto>")]
pub async fn delete_post(_db: &State<PgPool>, uuid_dto: &str, key: Result<JWT, NetworkResponse>,) -> Result<Value, NetworkResponse> {
  let _jwt = key?;

  let response = post_service::delete_post(_db, uuid_dto).await;

  match response {
    Ok(value) => Ok(value),
    Err(err) => Err(err),
  }
}

#[put("/post", data = "<put_request>")]
pub fn update_post(_db: &State<PgPool>, put_request: Json<UpdatePostDto<'_>>) -> Value {
    let mut conn = _db.get().unwrap();

    let result = diesel::update(posts.find(put_request.id))
        .set((title.eq(put_request.title), body.eq(put_request.body)))
        .returning(Post::as_returning())
        .get_result(&mut conn)
        .unwrap();

    json!(result)
}
