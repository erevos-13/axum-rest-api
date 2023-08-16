use crate::my_errors::MyError;
use crate::my_errors::MyError::*;
use crate::responces::UserListResponse;
use crate::structs::user::{User, UserModel};
use chrono::prelude::*;

use mongodb::bson::{doc, oid::ObjectId, Document};
use mongodb::options::{FindOneAndUpdateOptions, FindOptions, IndexOptions, ReturnDocument};
use mongodb::{bson, options::ClientOptions, Client, Collection, IndexModel};

#[derive(Clone, Debug)]
pub struct DB {
    pub user_collection: Collection<User>,
}
type Result<T> = std::result::Result<T, MyError>;

impl DB {
    pub async fn init() -> Result<Self> {
        let mongodb_uri = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
        let database_name =
            std::env::var("MONGO_INITDB_DATABASE").expect("MONGO_INITDB_DATABASE must be set.");
        let collection_name = std::env::var("MONGODB_USERS_COLLECTION")
            .expect("MONGODB_USERS_COLLECTION must be set.");

        let mut client_options = ClientOptions::parse(mongodb_uri)?;
        client_options.app_name = Some(database_name.to_string());

        let client = Client::with_options(client_options)?;
        let database = client.database(database_name.as_str());

        let user_collection = database.collection(collection_name.as_str());

        println!("✅ Database connected successfully");

        Ok(Self { user_collection })
    }
}
