use crate::routes::handlers;
use actix_web::web;

pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/home") // Setting the prefix for all routes
            .service(handlers::home_handler::hello) // Setting the "hello" route
            .service(handlers::home_handler::test), // Setting the "test" route
    );
}
