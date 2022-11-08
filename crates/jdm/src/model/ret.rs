use serde::{Deserialize, Serialize};
use actix_web::{Responder, HttpResponse};
use actix_http::StatusCode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ret<T> {
    pub status: bool,
    pub msg: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub data: Option<T>
}

impl <T> Ret<T>
where T: Serialize {
    pub fn to_ok_responder(self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }
    pub fn to_error_responder(self) -> HttpResponse {
        HttpResponse::InternalServerError().json(self)
    }
    pub fn to_responder(self, status_code: StatusCode) -> HttpResponse {
        HttpResponse::build(status_code).json(self)
    }
    pub fn set_status(mut self, status: bool) -> Self {
        self.status = status;
        self
    }
    pub fn set_data(mut self, data: Option<T>) -> Self {
        self.data = data;
        self
    }
    pub fn set_msg(mut self, msg: &str) -> Self {
        self.msg = msg.to_string();
        self
    }
}

pub fn build_data<T>(data: Option<T>) -> Ret<T>{
    Ret{
        status: true,
        msg: "ok".to_string(),
        data: data
    }
}

pub fn build_success<T>() -> Ret<T>{
    Ret{
        status: true,
        msg: "ok".to_string(),
        data: None
    }
}
pub fn build_failure<T>() -> Ret<T>{
    Ret{
        status: false,
        msg: "error".to_string(),
        data: None
    }
}