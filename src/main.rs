pub mod entities;
pub mod env;
pub mod health;

use std::{error::Error, net::Ipv4Addr};

use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_web::{HttpRequest, HttpResponse};
use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Url};

use env::env_port_number;
use health::{health_config, HealthCheck, HealthLevel};

#[actix_web::main]
async fn main() -> Result<(), impl Error> {
    env_logger::init();

    let port_number = env_port_number();
    println!(
        "Listening on port {port_number} eg. http://localhost:{port_number}",
        port_number = port_number
    );
    println!(
        "Swagger UI eg. http://localhost:{}/swagger-ui/",
        port_number
    );

    #[derive(OpenApi)]
    #[openapi(paths(health::check_health), components(schemas(HealthCheck, HealthLevel)))]
    struct ApiDocV1;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(hello))
            .service(web::scope("/api").configure(health_config))
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").urls(vec![(
                Url::new("Version 1", "/api-doc/openapi.json"),
                ApiDocV1::openapi(),
            )]))
    })
    .bind((Ipv4Addr::UNSPECIFIED, port_number))?
    .run()
    .await
}

pub(self) async fn hello(_req: HttpRequest) -> HttpResponse<BoxBody> {
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body("Hello World!".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{
        http::{self, header::ContentType},
        test,
    };
    #[actix_web::test]
    async fn test_index_ok() {
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_http_request();
        let resp = hello(req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}
