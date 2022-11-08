use std::fmt::Display;
use actix_web::HttpRequest;
use futures_util::future::LocalBoxFuture;
use serde::{Serializer};
use mongodb::bson::oid::ObjectId;
use chrono::{Utc, DateTime, FixedOffset} ;

#[derive(Debug)]
pub struct CustomError(String);

impl <'a> std::error::Error for CustomError {
}

impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl CustomError {
    pub fn build_dyn(message: &str) -> Box<dyn std::error::Error> {
        Box::new(Self(message.to_string()))
    }
}

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub type FutureResult<T> = LocalBoxFuture<'static, Result<T>>;


pub fn serialize_object_id<S>(object_id: &Option<ObjectId>, serializer: S) -> core::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match object_id {
      Some(ref object_id) => serializer.serialize_some(object_id.to_string().as_str()),
      None => serializer.serialize_none()
    }
}

pub fn datetime_format_localtime<S>(time: &DateTime<Utc>, serializer: S) -> core::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let local = time.with_timezone(&FixedOffset::east(8*3600));
    let str = local.format("%Y-%m-%d %H:%M:%S").to_string();
    serializer.serialize_str(str.as_str())
}

pub fn get_header_value<'a>(req: &'a HttpRequest, key: &'a str) -> Option<&'a str> {
    req.headers().get(key)?.to_str().ok()
}