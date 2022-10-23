#[macro_use] extern crate rocket;

use crate::routes::contacts;

mod routes;
mod models;

#[launch]
pub fn rocket_build() -> _ {
    rocket::build()
        .attach(contacts::stage())
}