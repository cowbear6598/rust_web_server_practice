use actix_web::{App, web, test::{init_service, TestRequest, call_and_read_body_json}};
use serde_json::{json};
use web_server_practice::{mongo, mongo::controllers, mongo::models};

#[actix_web::test]
async fn should_create_wheel_success() {
    let client = mongo::mongo_connect().await;

    models::wheel::set_wheel_field_unique(&client).await;

    let app = init_service(
        App::new()
            .app_data(web::Data::new(client))
            .service(controllers::wheel::create)
    ).await;

    let req = TestRequest::post()
        .uri("/api/wheel/create")
        .set_json(get_test_wheel())
        .to_request();

    let res: serde_json::Value = call_and_read_body_json(&app, req).await;

    assert_eq!(res, json!({
        "status": 0,
        "message": "ok",
        "data": {
            "wheel_id": res["data"]["wheel_id"],
            "is_public": false,
            "name": "test",
            "prizes":[]
        }
    }));
}

fn get_test_wheel() -> models::wheel::Wheel {
    models::wheel::Wheel {
        wheel_id: None,
        name: "test".to_string(),
        is_public: None,
        prizes: Vec::new(),
    }
}