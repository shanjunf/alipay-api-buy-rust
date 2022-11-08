use std::rc::Rc;

use actix_web::{get, web::{self, Form, Query}, post, Scope, Responder, HttpRequest};
use crate::rest::order;
use serde::{ Deserialize };
use crate::model::{ret, Ret, Page, Order};
use crate::common::{ CustomError, get_header_value};

use actix_web::{
    dev::{self, Service},
    error, Error,
};

#[derive(Deserialize, Debug)]
pub struct QueryInfo {
    product_id: Option<String>,
    tab: Option<i32>,
    page: Option<i32>,
    count: Option<i32>,
}

pub const RESOURCE: &str = "/orders";

#[get("")]
pub async fn gets(query: Query<QueryInfo>, req: HttpRequest) -> impl Responder {
    let user_id = get_header_value(&req, "token");
    if user_id.is_none() {
        return ret::build_failure::<String>().set_msg("无权限").to_ok_responder();
    }

    let tab = query.tab.unwrap_or(0);
    let page = query.page.unwrap_or(1);
    let size = query.count.unwrap_or(20);

    let ret = order::get_order_page(user_id.unwrap().to_string(), tab, page, size).await;

    match ret {
        Ok(page_obj) => {
            return ret::build_data(Some(page_obj)).to_ok_responder();
        },
        Err(e) => {
            if e.is::<CustomError>() {
                return ret::build_failure::<String>().set_msg(e.to_string().as_str()).to_ok_responder();
            }
            else {
                return ret::build_failure::<String>().set_msg("系统异常").to_ok_responder();
            }
        }
    }

}

#[post("")]
pub async fn create_order(body: Form<QueryInfo>, req: HttpRequest) -> impl Responder {
    let appid = get_header_value(&req, "appid");
    if appid.is_none() {
        return ret::build_failure::<String>().set_msg("无效的appid").to_ok_responder();
    }

    let user_id = get_header_value(&req, "token");
    if user_id.is_none() {
        return ret::build_failure::<String>().set_msg("无权限").to_ok_responder();
    }

    let appid = appid.unwrap().to_string();
    let user_id = user_id.unwrap().to_string();

    if let Some(product_id) = body.product_id.as_ref() {
        
        let order_ret = order::create_order(appid, user_id,product_id.to_string()).await;

        match order_ret {
            Ok(x) => {
                return ret::build_data(Some(x)).to_ok_responder();
            },
            Err(e) => {
                log::error!("{}", e);
                return ret::build_failure::<String>().set_msg("创建订单失败").to_ok_responder();
            }
        }
    }
    else {
        ret::build_failure::<String>().set_msg("product_id不能为空").to_ok_responder()
    }
}

#[derive(Deserialize, Debug)]
pub struct AlipayNotifyBody {
    out_trade_no: String,
    trade_no: String,
    trade_status: String,
    total_amount: String,
    buyer_email: String,
    sign: String
}

#[post("/notify/alipay")]
pub async fn notify_alipay(body: Form<AlipayNotifyBody>) -> impl Responder {
    let ret = order::order_payed(body.out_trade_no.clone(), body.trade_no.clone(), body.trade_status.clone(), body.total_amount.clone(), body.buyer_email.clone(), body.sign.clone()).await;
    match ret {
        Ok(x) => return x,
        Err(e) => {
            log::error!("{}", e);
            return "fail".to_string();
        }
    }
}
pub trait OrderScope {
    fn load_order_services(self) -> Scope;
}

impl OrderScope for Scope {
    fn load_order_services(self) -> Scope {
        self.service(
            web::scope(RESOURCE)
                .service(create_order)
                .service(gets)
                .service(notify_alipay)
            )
    }
}
