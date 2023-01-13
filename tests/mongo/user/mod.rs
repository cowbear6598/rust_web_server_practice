extern crate web_server_practice;

use actix_web::{App, web, test::{init_service, TestRequest, call_and_read_body_json}};
use actix_web::dev::{Service, ServiceResponse};
use web_server_practice::{mongo, mongo::controllers, mongo::models};

#[actix_web::test]
async fn should_mongo_user_controllers_success() {
    let client = mongo::mongo_connect().await;

    let app = init_service(
        App::new()
            .app_data(web::Data::new(client))
            .service(controllers::user::add_user)
            .service(controllers::user::get_user)
    ).await;

    let user = models::user::User {
        name: "test".to_string(),
        email: "cowbear6598@gmail.com".to_string(),
    };

    should_create_user_success(&app, &user).await;
}

async fn should_create_user_success(app: &impl Service<Request, Response=ServiceResponse<_>, Error=_, Future=_> + Sized, user: &models::user::User) {
    let user_create_req = TestRequest::post()
        .uri("/api/user/add")
        .set_json(&user)
        .to_request();

    let user_create_res: models::Response<models::user::User> = call_and_read_body_json(&app, user_create_req).await;

    assert_eq!(user_create_res, models::Response {
        status: 0,
        message: "ok".to_string(),
        data: models::user::User {
            name: "test".to_string(),
            email: "cowbear6598@gmail.com".to_string(),
        },
    });
}

async fn should_get_user_success() {

}


