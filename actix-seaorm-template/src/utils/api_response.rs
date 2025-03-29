use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, Responder, web};

pub struct ApiResponse {
    pub status_code: u16,
    pub body: String,
    response_code: StatusCode,
}

impl ApiResponse {
    pub fn new(status_code: u16, body: String) -> ApiResponse {
        ApiResponse {
            status_code,
            body,
            response_code: StatusCode::from_u16(status_code).unwrap(),
        }
    }
}

impl Responder for ApiResponse {
    type Body = BoxBody;
    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse {
        let body = BoxBody::new(web::BytesMut::from(self.body.as_bytes()));
        HttpResponse::new(self.response_code).set_body(body)
    }
}
