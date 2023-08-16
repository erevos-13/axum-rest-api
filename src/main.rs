mod db;
mod lid;
mod my_errors;
mod responces;
mod routes;
mod services;
mod structs;
mod validate_json;
use dotenv::dotenv;

use crate::lid::run;

#[tokio::main]
async fn main() {
    dotenv().ok();
    run().await;
}
