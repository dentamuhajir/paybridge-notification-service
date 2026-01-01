use actix_web::{post, web, HttpResponse};
use crate::domain::notification::service::NotificationService;

#[post("/notifications/test-email")]
pub async fn test_email(
    service: web::Data<NotificationService>,
) -> HttpResponse {
    match service.send_test_email("test@paybridge.local") {
        Ok(_) => HttpResponse::Ok().body("Dummy email sent"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
