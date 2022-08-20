use actix_web::body::BoxBody;
use actix_web::{web, HttpRequest, HttpResponse};
use serde::Serialize;
use utoipa::Component;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(handlers(check_health))]
struct ApiDoc;

#[derive(Serialize, Component)]
pub(super) enum HealthLevel {
    Operational,
    Deminished,
    Critical,
}

#[derive(Component, Serialize)]
pub(super) struct HealthCheck {
    #[component(example = "Operational")]
    rest_api: HealthLevel,
}

pub(super) fn health_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/health").route(web::get().to(check_health)));
}

#[utoipa::path(
    get,
    path = "/api/health",
    responses(
        (status = 200, description = "Checks the health of the service", body = HealthCheck)
    )
)]
pub(super) async fn check_health(_req: HttpRequest) -> HttpResponse<BoxBody> {
    let obj = HealthCheck {
        rest_api: HealthLevel::Operational,
    };
    HttpResponse::Ok().json(web::Json(obj))
}

#[cfg(test)]
mod tests {
    use actix_web::{
        http::{self, header::ContentType},
        test,
    };

    use super::*;

    #[actix_web::test]
    async fn test_index_ok() {
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_http_request();

        let resp = check_health(req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}
