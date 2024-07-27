use crate::schema::posts;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

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
pub struct NewPostDto<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatePostDto<'a> {
    pub id: i32,
    pub title: &'a str,
    pub body: &'a str,
    pub published: bool,
}
