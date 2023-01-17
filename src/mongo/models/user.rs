use mongodb::{Client, options::IndexOptions, IndexModel, bson::doc};
use serde::{Serialize, Deserialize};
use crate::mongo::{DB_NAME};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub uid: Option<String>,
    pub name: String,
    pub email: String,
}

pub async fn set_uid_and_email_unique(client: &Client){
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc! {"uid": 1, "email": 1})
        .options(options)
        .build();

    client.database(DB_NAME)
        .collection::<User>("user")
        .create_index(model, None)
        .await
        .expect("set uid and email unique failed");
}