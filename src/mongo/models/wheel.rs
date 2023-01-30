use mongodb::{Client, IndexModel, options::IndexOptions};
use mongodb::bson::doc;
use serde::{Serialize, Deserialize};
use crate::mongo::{DB_NAME};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Wheel {
    pub wheel_id: Option<String>,
    pub name: String,
    pub is_public: Option<bool>,
    pub prizes: Vec<Prize>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Prize {
    pub name: String,
    pub icon_url: String,
    pub count: u16,
}

pub async fn set_wheel_field_unique(client: &Client) {
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc! {"wheel_id": 1})
        .options(options)
        .build();

    client.database(DB_NAME)
        .collection::<Wheel>("wheel")
        .create_index(model, None)
        .await
        .expect("set wheel unique field failed");
}