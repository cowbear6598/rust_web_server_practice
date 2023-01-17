extern crate web_server_practice;

use std::time::Duration;
use actix_web::{App, web, test::{init_service, TestRequest, call_and_read_body_json}};
use serde_json::{json};
use web_server_practice::{mongo, mongo::controllers, mongo::models};

#[actix_web::test]
async fn should_register_success_and_cannot_has_same_user() {
    let client = mongo::mongo_connect().await;

    models::user::set_user_field_unique(&client).await;

    let app = init_service(
        App::new()
            .app_data(web::Data::new(client))
            .service(controllers::user::register)
    ).await;

    let user = get_test_user();

    let req = TestRequest::post()
        .uri("/api/user/register")
        .set_json(&user)
        .to_request();

    let res: serde_json::Value = call_and_read_body_json(&app, req).await;

    assert_eq!(res, json!({
        "status": 0,
        "message": "ok",
        "data": res["data"]
    }));

    let req = TestRequest::post()
        .uri("/api/user/register")
        .set_json(&user)
        .to_request();

    let res: serde_json::Value = call_and_read_body_json(&app, req).await;

    assert_eq!(true, res["message"].to_string().contains("duplicate key"));
}

#[actix_web::test]
async fn should_user_login_success() {
    tokio::time::sleep(Duration::from_secs(1)).await;

    let client = mongo::mongo_connect().await;
    let app = init_service(
        App::new()
            .app_data(web::Data::new(client))
            .service(controllers::user::login)
    ).await;

    let user = get_test_user();

    let req = TestRequest::post()
        .uri("/api/user/login")
        .set_json(&user)
        .to_request();

    let res: serde_json::Value = call_and_read_body_json(&app, req).await;

    assert_eq!(res, json!({
        "status": 0,
        "message": "ok",
        "data": res["data"]
    }));
}

#[actix_web::test]
async fn should_get_user_success() {
    tokio::time::sleep(Duration::from_secs(1)).await;

    let client = mongo::mongo_connect().await;

    let app = init_service(
        App::new()
            .app_data(web::Data::new(client))
            .service(controllers::user::get_user)
    ).await;

    let user = get_test_user();

    let req = TestRequest::get()
        .uri(&format!("/api/user/info/{}", &user.email))
        .to_request();

    let res: serde_json::Value = call_and_read_body_json(&app, req).await;

    assert_eq!(res, json!({
        "status": 0,
        "message": "ok",
        "data": res["data"]
    }));
}

#[actix_web::test]
async fn should_get_no_user_success() {
    tokio::time::sleep(Duration::from_secs(3)).await;

    let client = mongo::mongo_connect().await;
    let app = init_service(
        App::new()
            .app_data(web::Data::new(client))
            .service(controllers::user::get_user)
    ).await;

    let user = get_test_user();

    let req = TestRequest::get()
        .uri(&format!("/api/user/info/{}", &user.email))
        .to_request();

    let res: serde_json::Value = call_and_read_body_json(&app, req).await;

    assert_eq!(res, json!({
        "status": 1,
        "message": "未找到使用者"
    }));
}

#[actix_web::test]
async fn should_delete_user_success() {
    tokio::time::sleep(Duration::from_secs(2)).await;

    let client = mongo::mongo_connect().await;

    let app = init_service(
        App::new()
            .app_data(web::Data::new(client))
            .service(controllers::user::delete_user)
    ).await;

    let user = get_test_user();

    let req = TestRequest::delete()
        .uri("/api/user/delete")
        .set_json(&user)
        .to_request();

    let res: serde_json::Value = call_and_read_body_json(&app, req).await;

    assert_eq!(res, json!({
        "status": 0,
        "message": "ok"
    }));
}

#[actix_web::test]
async fn should_delete_no_user_success() {
    tokio::time::sleep(Duration::from_secs(3)).await;

    let client = mongo::mongo_connect().await;
    let app = init_service({
        App::new()
            .app_data(web::Data::new(client))
            .service(controllers::user::delete_user)
    }).await;

    let user = get_test_user();

    let req = TestRequest::delete()
        .uri("/api/user/delete")
        .set_json(&user)
        .to_request();

    let res: serde_json::Value = call_and_read_body_json(&app, req).await;

    assert_eq!(res, json!({
        "status": 1,
        "message": "未找到使用者"
    }));
}

fn get_test_user() -> models::user::User {
    models::user::User {
        uid: None,
        name: "test".to_string(),
        email: "cowbear6598@gmail.com".to_string(),
        phone: "0912345678".to_string(),
        account: "test".to_string(),
        password: "test654321".to_string()
    }
}
