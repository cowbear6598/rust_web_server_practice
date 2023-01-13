mod user;

use web_server_practice::mongo;
use mongodb::{bson::doc};

#[actix_web::test]
pub async fn should_connect_to_mongo_success() {
    let client = mongo::mongo_connect().await;

    client.database(mongo::DB_NAME)
        .run_command(doc! {"ping":1}, None)
        .await
        .expect("failed to connect to mongodb");
}