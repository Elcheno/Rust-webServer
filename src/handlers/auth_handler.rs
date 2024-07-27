use crate::lib::auth::auth_user;
use crate::models::response::{NetworkResponse, Response, ResponseBody};
use rocket::serde::json::{json, Value};

#[post("/login")]
pub async fn login_user() -> Result<Value, NetworkResponse> {
    let token = auth_user()?;

    let response = Response {
        body: ResponseBody::AuthToken(token),
    };

    Ok(json!(response))
}
