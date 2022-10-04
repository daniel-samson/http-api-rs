use actix_web::body::BoxBody;
use actix_web::{web, HttpRequest, HttpResponse};
use futures::executor::block_on;
use log::error;
use sea_orm::{ActiveValue, Database, DbErr, EntityTrait};
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};
use utoipa::ToSchema;

use crate::entities::{prelude::*, *};
use crate::env::env_database_url;   

#[derive(Serialize, ToSchema)]
pub(super) enum HealthLevel {
    Operational,
    Deminished,
    Critical,
}

#[derive(Serialize, ToSchema)]
pub(super) struct HealthCheck {
    rest_api: HealthLevel,
    database: HealthLevel,
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
    let mut health = HealthCheck {
        rest_api: HealthLevel::Operational,
        database: HealthLevel::Operational,
    };

    if let Err(error) = block_on(check_database_connection()) {
        health.database = HealthLevel::Critical;
        error!("database health critical: {}", error);
    }

    if let Err(error) = block_on(check_database_query()) {
        health.database = HealthLevel::Deminished;
        error!("database health diminished: {}", error);
    }

    HttpResponse::Ok().json(web::Json(health))
}

async fn check_database_connection() -> Result<(), DbErr> {
    Database::connect(env_database_url()).await?;

    Ok(())
}

async fn check_database_query() -> Result<(), DbErr> {
    let db = Database::connect(env_database_url()).await?;

    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let model = audit::ActiveModel {
        id: ActiveValue::NotSet,
        created_at: ActiveValue::Set(since_the_epoch.as_secs_f64().to_string()),
        data: ActiveValue::Set("{\"type\":\"healthcheck\"}".to_owned()),
    };

    Audit::insert(model).exec(&db).await?;

    Ok(())
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
