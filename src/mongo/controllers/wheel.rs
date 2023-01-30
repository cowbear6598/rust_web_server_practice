use actix_web::{delete, HttpResponse, post, get, web};
use mongodb::{bson::{doc, uuid::Uuid}, Client, Collection};
use serde_json::{json};
use crate::mongo::{DB_NAME, models};

const COLL_NAME: &str = "wheel";

#[post("api/wheel/create")]
pub async fn create(client: web::Data<Client>, form: web::Json<models::wheel::Wheel>) -> HttpResponse {
    let collection: Collection<models::wheel::Wheel> = client.database(DB_NAME).collection(COLL_NAME);

    let mut req_data = form.into_inner();

    req_data.wheel_id = Some(Uuid::new().to_string());
    req_data.is_public = Some(false);

    let result = collection.insert_one(req_data.clone(), None).await;

    match result {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": 0,
            "message": "ok",
            "data": {
                "wheel_id": req_data.wheel_id,
                "name": req_data.name,
                "is_public": req_data.is_public,
                "prizes": req_data.prizes
            }
        })),
        Err(err) => HttpResponse::InternalServerError().json(json!({
            "status": 2,
            "message": err.to_string()
        }))
    }
}

#[delete("api/wheel/delete")]
pub async fn delete(client: web::Data<Client>, form: web::Json<models::wheel::WheelDelete>) -> HttpResponse {
    let collection: Collection<models::wheel::WheelDelete> = client.database(DB_NAME).collection(COLL_NAME);
    let req_data = form.into_inner();

    let result = collection.find_one(doc! {"wheel_id": &req_data.wheel_id}, None).await;

    match result {
        Ok(Some(wheel)) => {
            let result = collection.delete_one(doc! {"wheel_id": &req_data.wheel_id}, None).await;

            match result {
                Ok(_)=>HttpResponse::Ok().json(json!({
                    "status": 0,
                    "message": "ok"
                })),
                Err(err) => HttpResponse::InternalServerError().json(json!({
                    "status": 2,
                    "message": err.to_string()
                }))
            }
        },
        Ok(None) => HttpResponse::NotFound().json(json!({
            "status": 1,
            "message": "未找到轉輪"
        })),
        Err(err) => HttpResponse::InternalServerError().json(json!({
            "status": 2,
            "message": err.to_string()
        }))
    }
}