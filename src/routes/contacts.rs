use mongodb::bson::doc;
use rocket::State;
use rocket::serde::json::{Json, json, Value};
use rocket::fairing::AdHoc;

use crate::lib::error::ApiError;
use crate::models::contact::{Contact, UpdateContact};
use crate::services::ContactService;

type ContactServiceState<'r> = &'r State<ContactService>;

#[post("/", format="json", data="<contact>")]
pub async fn create(
    contact: Json<Contact>, 
    contact_service: ContactServiceState<'_>
) -> Result<Json<Contact>, ApiError> {
    match contact_service.create(contact.into_inner()).await {
        Ok(contact) => Ok(Json(contact)),
        Err(err) => Err(err)
    }
}

#[get("/<id>")]
pub async fn find(
    id: String, 
    contact_service: ContactServiceState<'_>
) -> Result<Json<Contact>, ApiError> {
    match contact_service.find_one(&id).await {
        Ok(contact) => Ok(Json(contact)),
        Err(err) => Err(err)
    }
}

#[get("/user/<user_id>")]
pub async fn find_by_user_id(
    user_id: String, 
    contact_service: ContactServiceState<'_>
) -> Json<Vec<Contact>> {
    let contacts = contact_service.find_all_by_user_id(&user_id).await;

    Json(contacts)
}

#[put("/<id>", format="json", data="<update_contact>")]
pub async fn update(
    id: String,
    update_contact: Json<UpdateContact>,
    contact_service: ContactServiceState<'_>
) -> Result<Json<Contact>, ApiError> {
    match contact_service.update(&id, update_contact.into_inner()).await {
        Ok(updated_contact) => Ok(Json(updated_contact)),
        Err(err) => Err(err)
    }
}

#[delete("/<id>")]
pub async fn remove(id: String, contact_service: ContactServiceState<'_>) -> Result<Value, ApiError> {
    match contact_service.delete(&id).await {
        Ok(_) => Ok(json!({"result": true})),
        Err(err) => Err(err)
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Contacts", |rocket| async {
        rocket.mount("/contacts", routes![
            create, find, find_by_user_id, update, remove
        ])
    })
}
