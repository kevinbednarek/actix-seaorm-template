use crate::utils::api_response;
use actix_web::{Responder, get, web};

#[get("/hello/{name}")]
pub async fn hello(name: web::Path<String>) -> impl Responder {
    api_response::ApiResponse::new(200, format!("Hello {}!", name))
}

#[get("/test")]
pub async fn test(name: web::Path<String>) -> impl Responder {
    api_response::ApiResponse::new(200, "This is a test".to_string())
}
