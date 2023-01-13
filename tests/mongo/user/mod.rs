extern crate web_server_practice;

use actix_web::{App, web, test::{init_service, TestRequest, call_and_read_body_json}};
use web_server_practice::{mongo, mongo::controllers, mongo::models};

#[actix_web::test]
async fn should_mongo_user_controllers_success() {
    let user = models::user::User {
        name: "test".to_string(),
        email: "cowbear6598@gmail.com".to_string(),
    };

    should_create_user_success(&user).await;
    should_get_user_success(&user).await;
}

async fn should_create_user_success(user: &models::user::User) {
    let client = mongo::mongo_connect().await;

    let app = init_service(
        App::new()
            .app_data(web::Data::new(client))
            .service(controllers::user::add_user)
    ).await;

    let req = TestRequest::post()
        .uri("/api/user/add")
        .set_json(&user)
        .to_request();

    let res: models::Response<models::user::User> = call_and_read_body_json(&app, req).await;

    assert_eq!(res, models::Response {
        status: 0,
        message: "ok".to_string(),
        data: models::user::User {
            name: "test".to_string(),
            email: "cowbear6598@gmail.com".to_string(),
        },
    });
}

async fn should_get_user_success(user: &models::user::User) {
    let client = mongo::mongo_connect().await;

    let app = init_service(
        App::new()
            .app_data(web::Data::new(client))
            .service(controllers::user::add_user)
            .service(controllers::user::get_user)
    ).await;

    let req = TestRequest::get()
        .uri(&format!("/api/user/info/{}", &user.email))
        .to_request();

    let res: models::Response<models::user::User> = call_and_read_body_json(&app, req).await;

    assert_eq!(res, models::Response {
        status: 0,
        message: "ok".to_string(),
        data: models::user::User {
            name: "test".to_string(),
            email: "cowbear6598@gmail.com".to_string(),
        },
    });
}


