#[macro_use] extern crate rocket;

use std::env;

use dotenv::dotenv;
use mongodb::Collection;

use crate::routes::contacts;
use crate::lib::Repository;
use crate::models::contact::Contact;
use crate::services::ContactService;
use crate::setup::setup_database;

mod setup;
mod routes;
mod models;
mod lib;
mod services;

#[launch]
pub async fn rocket_build() -> _ {
    dotenv().ok();

    let uri = env::var("MONGODB")
        .expect("Error loading env variable (MONGODB)");

    let db = setup_database(uri).await;

    let contacts_collection: Collection<Contact> = db.collection("contacts");
    let contact_repository: Repository<Contact> = Repository::new(contacts_collection);
    let contact_service: ContactService = ContactService::new(contact_repository);

    rocket::build()
        .attach(contacts::stage())
        .manage(contact_service)
}