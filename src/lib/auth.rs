use crate::lib::jwt::create_jwt;
use crate::models::response::NetworkResponse;

pub fn auth_user() -> Result<String, NetworkResponse> {
    let user_id = 5;

    match create_jwt(user_id) {
        Ok(token) => Ok(token),
        Err(err) => Err(NetworkResponse::BadRequest(err.to_string())),
    }
}
