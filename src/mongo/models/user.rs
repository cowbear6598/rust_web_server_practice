use mongodb::{Client, options::IndexOptions, IndexModel, bson::doc};
use serde::{Serialize, Deserialize};
use crate::mongo::{DB_NAME};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub uid: Option<String>,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub account: String,
    pub password: String,
    pub created_date: Option<String>,
    pub last_login_date: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct UserLogin {
    pub account: String,
    pub password: String
}

#[derive(Serialize, Deserialize)]
pub struct UserDelete {
    pub uid: String
}

pub async fn set_user_field_unique(client: &Client) {
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc! {"uid": 1, "email": 1, "account": 1, "phone": 1})
        .options(options)
        .build();

    client.database(DB_NAME)
        .collection::<User>("user")
        .create_index(model, None)
        .await
        .expect("set user unique field failed");
}