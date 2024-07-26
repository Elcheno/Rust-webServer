use crate::schema::posts;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct new_post<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct new_post_dto<'a> {
    pub title: &'a str,
    pub body: &'a str,
}
