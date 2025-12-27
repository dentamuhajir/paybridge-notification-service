use actix_web::{web};

use crate::domain::health::routes::health_routes;


pub fn register_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(health_routes());
        // .service(notification_routes());
}
