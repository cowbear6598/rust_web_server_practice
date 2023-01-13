use actix_web::{get, HttpResponse, post, web};
use mongodb::{bson::doc, Client, Collection};
use crate::mongo::models;

const DB_NAME: &str = "rust_test";
const COLL_NAME: &str = "user";

#[post("api/user/add")]
pub async fn add_user(client: web::Data<Client>, form: web::Json<models::user::User>) -> HttpResponse {
    let req_data = form.into_inner();

    let collection = client.database(DB_NAME).collection(COLL_NAME);
    let result = collection.insert_one(req_data.clone(), None).await;

    let response = models::Response {
        status: 0,
        message: "ok".to_string(),
        data: req_data.clone(),
    };

    match result {
        Ok(_) => HttpResponse::Ok().json(response),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

#[get("api/user/info/{email}")]
pub async fn get_user(client: web::Data<Client>, email: web::Path<String>) -> HttpResponse {
    let email = email.into_inner();
    let collection: Collection<models::user::User> = client.database(DB_NAME).collection(COLL_NAME);
    let result = collection.find_one(doc! {"email": email}, None).await;

    match result {
        Ok(Some(user)) => HttpResponse::Ok().json(models::Response {
            status: 0,
            message: "ok".to_string(),
            data: user
        }),
        Ok(None) => HttpResponse::NotFound().json(models:: Response{
            status: 1,
            message: "none".to_string(),
            data: "".to_string()
        }),
        Err(err) => HttpResponse::InternalServerError().json(models:: Response{
            status: 100,
            message: err.to_string(),
            data: "".to_string()
        })
    }
}