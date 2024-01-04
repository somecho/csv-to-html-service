use actix_web::error::{Error, ErrorUnsupportedMediaType};
use actix_web::middleware::Logger;
use actix_web::{http::header::ContentType, post, web, App, HttpRequest, HttpResponse, HttpServer};
use env_logger::Env;
use serde_derive::Deserialize;

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct Params {
    hasHeaders: Option<bool>,
    delimiter: Option<String>,
}

#[post("/")]
async fn convert_csv_html(request: HttpRequest, body: String) -> Result<HttpResponse, Error> {
    let params = web::Query::<Params>::from_query(request.query_string()).unwrap();
    let csv_has_headers = match params.hasHeaders {
        Some(b) => b,
        None => true,
    };
    let delimiter = match &params.delimiter {
        Some(d) => d.as_bytes()[0],
        None => b',',
    };

    let html = csv_to_html::convert(&body, &delimiter, &csv_has_headers);
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
    HttpServer::new(|| App::new().wrap(Logger::default()).service(convert_csv_html))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
