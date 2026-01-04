use actix_web::web;
use std::sync::Arc;

use crate::domain::notification::{
    routes::test_email,
    service::NotificationService,
};

use crate::infrastructure::mail::mailhog::MailhogEmailSender;

pub fn configure_notification(cfg: &mut web::ServiceConfig) {
    let email_sender = Arc::new(MailhogEmailSender::new());
    let notification_service = NotificationService::new(email_sender);

    cfg
        .app_data(web::Data::new(notification_service))
        .service(test_email);
}
