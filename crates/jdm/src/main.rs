use actix_web::{ web, App, HttpServer, Scope};
use actix_web;
use alipay_sdk_rust::pay::Payer;
use alipay_sdk_rust::biz::SystemOAuthTokenBiz;
use chrono::{FixedOffset, Utc};
use mongodb::options::{ FindOneOptions };
use rand::random;
mod middleware;
mod controller;
mod rest;
mod service;
mod config;
mod model;
mod common;
mod lib;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::rc::Rc;
use base64::{encode, decode};
use std::iter::repeat;

use model:: {User, Page};
use lib::{alipay, util};

use controller::user::UserScope;
use controller::order::OrderScope;

use std::{
    fs,
    io::{Error, ErrorKind, Result},
};

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    model::init_mongo_client().await;
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("main function invoke~");

    HttpServer::new(|| {
        App::new()
        .wrap(actix_web::middleware::Compress::default())
        .wrap(actix_web::middleware::Logger::default())
        .wrap(middleware::Auth::default())
        .service(web::scope(config::SCOPE_API)
            .load_user_services()
            .load_order_services())
    }).bind(("0.0.0.0", 9000))?
    .run()
    .await 

}