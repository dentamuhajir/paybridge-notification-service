use actix_web::{App, HttpResponse, HttpServer, Responder, get};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server started on http:localhost:8080");

    HttpServer::new(|| {
        App::new()
        .service(hello) // register / endpoint
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await

}
