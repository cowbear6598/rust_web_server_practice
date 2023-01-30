use actix_web::{delete, HttpResponse, post, web};
use mongodb::{bson::{doc, uuid::Uuid}, Client, Collection};
use serde_json::{json};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::mongo::{DB_NAME, models};

const COLL_NAME: &str = "user";
const SALT_ROUND: [u8; 16] = [12; 16];

#[post("api/user/register")]
pub async fn register(client: web::Data<Client>, form: web::Json<models::user::User>) -> HttpResponse {
    let collection: Collection<models::user::User> = client.database(DB_NAME).collection(COLL_NAME);

    let mut req_data = form.into_inner();

    let result = collection.find_one(doc! {"$or":[
        {"account": &req_data.account},
        {"email": &req_data.email},
        {"phone": &req_data.phone}
    ]}, None).await;

    if !result.unwrap().is_none() {
        return HttpResponse::InternalServerError().json(json!({
            "status": 1,
            "message": "此帳號已被註冊"
        }))
    }

    req_data.uid = Some(Uuid::new().to_string());
    req_data.password = bcrypt::hash_with_salt(&req_data.password, bcrypt::DEFAULT_COST, SALT_ROUND).unwrap().to_string();

    let start = SystemTime::now();
    let utc_time = start.duration_since(UNIX_EPOCH).expect("time went backwards");

    req_data.created_date = Some(utc_time.as_secs().to_string());
    req_data.last_login_date = req_data.created_date.clone();

    let result = collection.insert_one(req_data.clone(), None).await;

    match result {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": 0,
            "message": "ok",
            "data": {
                "uid": req_data.uid,
                "name": req_data.name,
                "email": req_data.email,
                "phone": req_data.phone,
                "created_date": req_data.created_date,
                "last_login_date": req_data.last_login_date
            }
        })),
        Err(err) => HttpResponse::InternalServerError().json(json!({
            "status": 1,
            "message": err.to_string()
        }))
    }
}

#[post("api/user/login")]
pub async fn login(client: web::Data<Client>, form: web::Json<models::user::UserLogin>) -> HttpResponse {
    let req = form.into_inner();

    let collection: Collection<models::user::User> = client.database(DB_NAME).collection(COLL_NAME);
    let result = collection.find_one(doc! {"account": &req.account}, None).await;

    match result {
        Ok(Some(user)) => {
            match bcrypt::verify(&req.password, &user.password) {
                Ok(_) => HttpResponse::Ok().json(json!({
                    "status": 0,
                    "message": "ok",
                    "data": user
                })),
                Err(_) => HttpResponse::InternalServerError().json(json!({
                    "status": 1,
                    "message": "帳號或密碼錯誤"
                }))
            }
        }
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
pub async fn delete(client: web::Data<Client>, form: web::Json<models::user::UserDelete>) -> HttpResponse {
    let req = form.into_inner();

    let collection: Collection<models::user::User> = client.database(DB_NAME).collection(COLL_NAME);

    let result = collection.find_one_and_delete(doc! {"uid": req.uid}, None).await;

    match result {
        Ok(Some(_)) => HttpResponse::Ok().json(json!({
            "status": 0,
            "message": "ok"
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