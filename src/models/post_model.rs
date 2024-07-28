use crate::schema::posts;
use diesel::prelude::*;
use rocket::serde::json::json;
use rocket::serde::{Deserialize, Serialize};

use super::response::{NetworkResponse, Response, ResponseBody};

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct NewPostDto<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct UpdatePostDto<'a> {
    pub id: i32,
    pub title: &'a str,
    pub body: &'a str,
    pub published: bool,
}

impl<'a> NewPostDto<'a> {
    pub fn validate(&self) -> Result<&NewPostDto<'a>, NetworkResponse> {
        if self.title.is_empty() {
            let response = Response {
                body: ResponseBody::Message("Title cannot be empty".into()),
            };
            return Err(NetworkResponse::BadRequest(json!(response).to_string()));
        }

        if self.body.is_empty() {
            let response = Response {
                body: ResponseBody::Message("Body cannot be empty".into()),
            };
            return Err(NetworkResponse::BadRequest(json!(response).to_string()));
        }

        Ok(self)
    }
}
