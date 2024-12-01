use actix_web::{web, App, HttpResponse, HttpServer};

mod app;
mod db;

async fn render_app_handler() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(app::render_app())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(render_app_handler)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
