use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Contact {
    pub name: String,
    pub phone: String,
    pub image: String,
    pub user_id: String
}

impl Contact {
    pub fn update(&mut self, update_contact: UpdateContact) {
        self.name = update_contact.name.unwrap_or(self.name.clone());
        self.phone = update_contact.phone.unwrap_or(self.phone.clone());
        self.image = update_contact.image.unwrap_or(self.image.clone());
        self.user_id = update_contact.user_id.unwrap_or(self.user_id.clone());
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct UpdateContact {
    pub name: Option<String>,
    pub phone: Option<String>,
    pub image: Option<String>,
    pub user_id: Option<String>
}
