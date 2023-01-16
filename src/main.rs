use actix_web::{HttpServer, App, web::Data, middleware::Logger};
use web_server_practice::mongo::{controllers, mongo_connect};

#[actix_web::main]
#[ignore = "requires MongoDB instance running"]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let client = mongo_connect().await;

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .app_data(Data::new(client.clone()))
            .wrap(logger)
            .service(controllers::user::add_user)
            .service(controllers::user::get_user)
            .service(controllers::user::delete_user)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
