use mongodb::{Collection};
use mongodb::bson::{self, Bson, oid::ObjectId, doc, Document};
use rocket::futures::TryStreamExt;
use rocket::serde::{Serialize, DeserializeOwned};
use thiserror::Error as ThisError;

pub struct Repository<T> {
    col: Collection<T>
}

impl<T> Repository<T> {
    pub fn new(col: Collection<T>) -> Self {
        Repository { col }
    }

    fn string_to_object_id(&self, id: &String) -> Result<ObjectId, Error> {
        match ObjectId::parse_str(id) {
            Ok(result_id) => Ok(result_id),
            Err(_) => Err(Error::invalid_argument("Invalid Id"))
        }
    }
}

impl<T> Repository<T>
where 
    T: Serialize + DeserializeOwned + Unpin + Send + Sync
{
    pub async fn create(&self, entry: T) -> Result<String, Error> {
        let result = self.col.insert_one(entry, None)
            .await
            .unwrap();

        Ok(result.inserted_id.as_object_id().unwrap().to_string())
    }

    pub async fn find(&self, filter: Document) -> Vec<T> {

        let cursor = match self.col.find(filter, None).await {
            Ok(cursor) => cursor,
            Err(_) => return vec![]
        };

        cursor.try_collect().await.unwrap_or_else(|_| vec![])
    }

    pub async fn find_one(&self, id: &String) -> Result<Option<T>, Error> {

        let object_id = match self.string_to_object_id(id) {
            Ok(result_id) => result_id,
            Err(err) => return Err(err)
        };

        let filter = doc! { "_id": object_id };

        match self.col.find_one(filter, None).await {
            Ok(result) => Ok(result),
            Err(err) => Err(Error::from(err))
        }
    }

    pub async fn update(&self, id: &String, new_doc: T) -> Result<Option<T>, Error> {

        let object_id = match self.string_to_object_id(id) {
            Ok(result_id) => result_id,
            Err(err) => return Err(err)
        };

        let filter = doc! { "_id": object_id };

        let bson_doc = self.to_bson(new_doc)?;

        let update = doc! { "$set": bson_doc };

        match self.col.find_one_and_update(filter, update, None).await {
            Ok(_) => self.find_one(id).await,
            Err(err) => Err(Error::from(err))
        }
    }

    pub async fn delete(&self, id: &String) -> Result<(), Error> {
        
        let object_id = match self.string_to_object_id(id) {
            Ok(result_id) => result_id,
            Err(err) => return Err(err)
        };

        let filter = doc! { "_id": object_id };

        self.col.delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting the object");
            
        Ok(())
    }

    fn to_bson(&self, document: T) -> Result<Bson, Error> {
        match bson::to_bson(&document) {
            Ok(result) => Ok(result),
            Err(err) => Err(Error::from(err))
        }
    }
}

#[derive(Clone, Debug, ThisError)]
#[error("{kind}")]
pub struct Error {
    pub kind: ErrorKind
}

impl Error {
    pub fn new(kind: ErrorKind) -> Error {
        Error {
            kind
        }
    }

    pub fn invalid_argument(message: impl Into<String>) -> Error {
        ErrorKind::InvalidArgument { 
            message: message.into() 
        }.into()
    }
}

impl<E> From<E> for Error
where
    ErrorKind: From<E>,
{
    fn from(err: E) -> Self {
        Error::new(err.into())
    }
}

impl From<mongodb::error::Error> for ErrorKind {
    fn from(err: mongodb::error::Error) -> Self {
        err.into()
    }
}

impl From<mongodb::bson::oid::Error> for ErrorKind {
    fn from(err: mongodb::bson::oid::Error) -> Self {
        Self::InvalidArgument {
            message: err.to_string()
        }
    }
}

impl From<mongodb::bson::ser::Error> for ErrorKind {
    fn from(err: mongodb::bson::ser::Error) -> Self {
        Self::BsonSeralization(err)
    }
}

#[derive(Clone, Debug, ThisError)]
pub enum ErrorKind {
    /// An invalid argument was provided.
    #[error("An invalid argument was provided: {message}")]
    InvalidArgument { message: String },

    /// Wrapper around `bson::ser::Error`.
    #[error("{0}")]
    BsonSeralization(mongodb::bson::ser::Error)
}
