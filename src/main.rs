use actix_web::{App, HttpServer};
use crate::infrastructure::config::Config;

mod domain;
mod infrastructure;
mod startup;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::load();

    let host = config.app_host.clone();
    let port = config.app_port;

    println!("Starting Paybridge Notification Service");
    println!("Server running at http://{host}:{port}");
    
    HttpServer::new(|| {
        App::new()
            .configure(startup::configure_notification)
    })
        .bind((host, port))?
        .run()
        .await
}
