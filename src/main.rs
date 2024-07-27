#[macro_use]
extern crate rocket;

pub mod handlers;
pub mod lib;
pub mod models;
pub mod schema;
pub mod service;

use dotenvy::dotenv;
use lib::connection::build_pool;

use handlers::auth_handler::*;
use handlers::post_handler::*;

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .mount(
            "/",
            routes![get_post, create_post, delete_post, update_post, login_user],
        )
        .manage(build_pool())
}
