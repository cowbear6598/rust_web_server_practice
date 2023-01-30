use actix_web::{HttpServer, App, web::{Data, scope}, middleware::Logger};
use web_server_practice::mongo::{controllers, mongo_connect, models};

#[actix_web::main]
#[ignore = "requires MongoDB instance running"]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let client = mongo_connect().await;

    models::user::set_user_field_unique(&client).await;
    models::wheel::set_wheel_field_unique(&client).await;

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .app_data(Data::new(client.clone()))
            .wrap(logger)
            .service(controllers::user::register)
            .service(controllers::user::login)
            .service(controllers::user::delete)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
