use std::{error::Error, net::Ipv4Addr};
use std::env;

use actix_web::{middleware::Logger, web, App, HttpServer};
use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Url};



#[actix_web::main]
async fn main() -> Result<(), impl Error> {
    env_logger::init();

    let port_number = env_port_number();
    println!("Listening on port {x} eg. http://localhost:{x}", x = port_number);
    println!("Swagger UI eg. http://localhost:{}/swagger-ui/", port_number);

    #[derive(OpenApi)]
    #[openapi(handlers(api1::hello1))]
    struct ApiDoc1;

    #[derive(OpenApi)]
    #[openapi(handlers(api2::hello2))]
    struct ApiDoc2;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .service(api1::hello1)
                    .service(api2::hello2),
            )
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").urls(vec![
                (
                    Url::new("api1", "/api-doc/openapi1.json"),
                    ApiDoc1::openapi(),
                ),
                (
                    Url::with_primary("api2", "/api-doc/openapi2.json", true),
                    ApiDoc2::openapi(),
                ),
            ]))
    })
    .bind((Ipv4Addr::UNSPECIFIED, port_number))?
    .run()
    .await
}

fn env_port_number() -> u16 {
    let key = "PORT";
    match env::var(key) {
        Ok(val) => val.parse().unwrap(),
        Err(_e) => 9090,
    }
}

mod api1 {
    use actix_web::get;

    #[utoipa::path(
        context_path = "/api",
        responses(
            (status = 200, description = "Hello from api 1", body = String)
        )
    )]
    #[get("/api1/hello")]
    pub(super) async fn hello1() -> String {
        "hello from api 1".to_string()
    }
}

mod api2 {
    use actix_web::get;

    #[utoipa::path(
        context_path = "/api",
        responses(
            (status = 200, description = "Hello from api 2", body = String)
        )
    )]
    #[get("/api2/hello")]
    pub(super) async fn hello2() -> String {
        "hello from api 2".to_string()
    }
}