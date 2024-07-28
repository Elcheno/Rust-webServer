use rocket::serde::json::{Json, Value};
use rocket::State;

use crate::models::jwt_model::JWT;
use crate::models::post_model::{NewPostDto, UpdatePostDto};
use crate::models::response::NetworkResponse;

use crate::lib::connection::PgPool;
use crate::service::post_service;

#[get("/post")]
pub async fn get_post(
    _db: &State<PgPool>,
    key: Result<JWT, NetworkResponse>,
) -> Result<Value, NetworkResponse> {
    let _jwt = key?;

    let response = post_service::get_posts(_db).await;

    match response {
        Ok(value) => Ok(value),
        Err(err) => Err(err),
    }
}

#[post("/post", data = "<body>")]
pub async fn create_post(
    _db: &State<PgPool>,
    body: Json<NewPostDto<'_>>,
    key: Result<JWT, NetworkResponse>,
) -> Result<Value, NetworkResponse> {
    let _jwt = key?;

    let post_request = body.into_inner();
    let post_request = post_request.validate()?;

    let response = post_service::create_post(_db, &post_request).await;

    match response {
        Ok(value) => Ok(value),
        Err(err) => Err(err),
    }
}

#[delete("/post/<uuid_dto>")]
pub async fn delete_post(
    _db: &State<PgPool>,
    uuid_dto: &str,
    key: Result<JWT, NetworkResponse>,
) -> Result<Value, NetworkResponse> {
    let _jwt = key?;

    let response = post_service::delete_post(_db, uuid_dto).await;

    match response {
        Ok(value) => Ok(value),
        Err(err) => Err(err),
    }
}

#[put("/post", data = "<put_request>")]
pub async fn update_post(
    _db: &State<PgPool>,
    put_request: Json<UpdatePostDto<'_>>,
    key: Result<JWT, NetworkResponse>,
) -> Result<Value, NetworkResponse> {
    let _jwt = key?;

    let response = post_service::update_post(_db, put_request).await;

    match response {
        Ok(value) => Ok(value),
        Err(err) => Err(err),
    }
}
