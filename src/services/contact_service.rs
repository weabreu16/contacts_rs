use mongodb::bson::doc;

use crate::lib::repository::{Repository, Error, ErrorKind};
use crate::lib::error::ApiError;
use crate::models::contact::{Contact, UpdateContact};

pub struct ContactService {
    contact_repository: Repository<Contact>
}

impl ContactService {
    pub fn new(contact_repository: Repository<Contact>) -> ContactService {
        ContactService { contact_repository }
    }

    pub async fn find_one(&self, id: &String) -> Result<Contact, ApiError> {
        let result = self.contact_repository.find_one(id).await;

        if let Err(error) = result {
            return Err(self.handle_errors(error));
        };

        match result.unwrap() {
            Some(contact) => Ok(contact),
            None => Err(ApiError::NotFound("Contact not found".to_string()))
        }
    }

    pub async fn find_all_by_user_id(&self, user_id: &String) -> Vec<Contact> {
        
        let filter = doc! { "user_id": user_id };

        self.contact_repository.find(filter).await
    }

    pub async fn create(&self, contact: Contact) -> Result<Contact, ApiError> {
        match self.contact_repository.create(contact).await {
            Ok(id) => {
                println!("{id}");
                self.find_one(&id).await
            },
            Err(err) => Err(self.handle_errors(err))
        }
    }

    pub async fn update(&self, id: &String, update_contact: UpdateContact) -> Result<Contact, ApiError> {
        let mut contact = match self.find_one(id).await {
            Ok(contact) => contact,
            Err(error) => return Err(error)
        };

        contact.update(update_contact);

        match self.contact_repository.update(id, contact).await {
            Ok(updated) => Ok(updated.unwrap()),
            Err(error) => Err(self.handle_errors(error))
        }
    }

    pub async fn delete(&self, id: &String) -> Result<(), ApiError> {
        let contact = self.find_one(id).await;

        if let Err(err) = contact {
            return Err(err)
        }

        match self.contact_repository.delete(id).await {
            Ok(_) => Ok(()),
            Err(err) => Err(self.handle_errors(err))
        }
    }

    fn handle_errors(&self, error: Error) -> ApiError {
        match error.kind {
            ErrorKind::InvalidArgument { message } => ApiError::BadRequest(message),
            _ => ApiError::InternalServer
        }
    }
}
