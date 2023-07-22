use mongodb::{options::ClientOptions, Client, Collection};

use crate::{
    my_errors::MyError,
    structs::{common::DatabaseConfig, user::User},
};

#[derive(Clone, Debug)]
pub struct DB {
    pub user_collection: Collection<User>,
}
type Result<T> = std::result::Result<T, MyError>;

impl DB {
    pub async fn init() -> Result<Self> {
        let database_config = DatabaseConfig::new();
        let mut client_options = ClientOptions::parse(database_config.uri).await?;

        // the server will select the algorithm it supports from the list provided by the driver
        let client = Client::with_options(client_options)?;
        let db = client.database("gym_mgmt");
        let user_collection = db.collection::<User>("users");
        println!("✅ Database connected successfully");

        Ok(Self { user_collection })
    }
}
