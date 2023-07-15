mod lid;
mod routes;

use crate::lid::run;

#[tokio::main]
async fn main() {
    run().await;
}
