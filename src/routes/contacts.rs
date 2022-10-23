use rocket::State;
use rocket::futures::lock::Mutex;
use rocket::serde::json::{Json, json, Value};
use rocket::fairing::AdHoc;

use crate::models::contact::{Contact, UpdateContact};

type ContactList = Mutex<Vec<Contact>>;
type Contacts<'r> = &'r State<ContactList>;

#[post("/", format="json", data="<contact>")]
pub async fn create(contact: Json<Contact>, list: Contacts<'_>) -> Value {
    let mut list = list.lock().await;

    let id = list.len();
    list.push(contact.into_inner());

    json!({
        "status": "ok",
        "id": id
    })
}

#[get("/<id>")]
pub async fn find(id: usize, list: Contacts<'_>) -> Option<Json<Contact>> {
    let list = list.lock().await;

    let finded_contact = list.get(id)?.clone();

    Some(Json(finded_contact))
}

#[get("/user/<user_id>")]
pub async fn find_by_user_id(user_id: String, list: Contacts<'_>) -> Option<Json<Vec<Contact>>> {
    let list = list.lock().await;

    let contact_list = list.iter()
        .filter(|contact| contact.user_id == user_id)
        .cloned()
        .collect();

    Some(Json(contact_list))
}

#[put("/<id>", format="json", data="<update_contact>")]
pub async fn update(
    id: usize,
    update_contact: Json<UpdateContact>,
    list: Contacts<'_>
) -> Option<Json<Contact>> {
    match list.lock().await.get_mut(id) {
        Some(existing) => {
            existing.update(update_contact.into_inner());

            let updated_contact = existing.clone();
            Some(Json(updated_contact))
        },
        None => None
    }
}

#[delete("/<id>")]
pub async fn remove(id: usize, list: Contacts<'_>) -> Value {
    let mut list = list.lock().await;

    list.remove(id);

    json!({"status": "ok"})
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Contacts", |rocket| async {
        rocket.mount("/contacts", routes![
            create, find, find_by_user_id, update, remove
        ])
            .manage(ContactList::new(vec![]))
    })
}
