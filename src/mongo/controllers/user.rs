use actix_web::{get, delete, HttpResponse, post, web};
use mongodb::{bson::doc, Client, Collection};
use crate::mongo::models;
use serde_json::{json};

const DB_NAME: &str = "rust_test";
const COLL_NAME: &str = "user";

#[post("api/user/add")]
pub async fn add_user(client: web::Data<Client>, form: web::Json<models::user::User>) -> HttpResponse {
    let req_data = form.into_inner();

    let collection = client.database(DB_NAME).collection(COLL_NAME);
    let result = collection.insert_one(req_data.clone(), None).await;

    match result {
        Ok(_) => HttpResponse::Ok().json(json!({
                "status": 0,
                "message": "ok"
            })),
        Err(err) =>
            HttpResponse::InternalServerError().json(json!({
                "status": 1,
                "message": err.to_string()
            }))
    }
}

#[get("api/user/info/{email}")]
pub async fn get_user(client: web::Data<Client>, email: web::Path<String>) -> HttpResponse {
    let email = email.into_inner();
    let collection: Collection<models::user::User> = client.database(DB_NAME).collection(COLL_NAME);
    let result = collection.find_one(doc! {"email": email}, None).await;

    match result {
        Ok(Some(user)) => HttpResponse::Ok().json(json!({
            "status": 0,
            "message": "ok",
            "data": user
        })),
        Ok(None) => HttpResponse::NotFound().json(json!({
            "status": 1,
            "message": "未找到使用者"
        })),
        Err(err) => HttpResponse::InternalServerError().json(json!({
            "status": 1,
            "message": err.to_string()
        }))
    }
}

#[delete("api/user/delete")]
pub async fn delete_user(client: web::Data<Client>, form: web::Json<models::user::User>) -> HttpResponse {
    let req = form.into_inner();

    let collection: Collection<models::user::User> = client.database(DB_NAME).collection(COLL_NAME);

    let result = collection.delete_one(doc! {"email": req.email}, None).await;

    match result {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": 0,
            "message": "ok"
        })),
        Err(err) => HttpResponse::InternalServerError().json(json!({
            "status": 1,
            "message": err.to_string()
        }))
    }
}