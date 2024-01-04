use actix_web::error::{Error, ErrorUnsupportedMediaType};
use actix_web::middleware::Logger;
use actix_web::{http::header::ContentType, post, App, HttpRequest, HttpResponse, HttpServer};
use env_logger::Env;

#[post("/")]
async fn gotcha(request: HttpRequest, body: String) -> Result<HttpResponse, Error> {
    let html = csv_to_html::convert(&body, &b',', &true);
    let error = ErrorUnsupportedMediaType("Content-Type must be set to 'text/csv'");
    match request.headers().get("content-type") {
        Some(content_type) => match content_type.to_str().unwrap() {
            "text/csv" => Ok(HttpResponse::Ok()
                .content_type(ContentType::html())
                .body(html)),
            _ => Err(error),
        },
        None => Err(error),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| App::new().wrap(Logger::default()).service(gotcha))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
