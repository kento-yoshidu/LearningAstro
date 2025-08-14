use actix_web::{web, App, HttpResponse, HttpServer};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/", web::get().to(|| async {
            HttpResponse::Ok().body("Hello World")
        }))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
