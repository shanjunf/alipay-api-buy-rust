use std::collections::HashMap;

use actix_http::header::HeaderValue;
use actix_web::{web, get, put, post, Scope, Responder, HttpRequest, HttpResponse, http::header};
use crate::service::{ UserService, ProductService, OrderService };
use serde::{Deserialize};
use serde_json::{self, json, Value};
use crate::model::{ User, Order, Page, Ret, OrderStatus };
use alipay_sdk_rust::biz::{self, TradeCreateBiz};
use alipay_sdk_rust::pay::{PayClient, Payer};
use crate::common::{Result, CustomError};
use base64::{ decode };
use crate::lib::{ alipay, util };
use crate::config;

pub async fn get_order_page(user_id: String, tab: i32, page: i32, size: i32) -> Result<Page<Order>> {
    OrderService::get_order_page_by_user_id(user_id, tab, page, size).await
}

pub async fn create_order(appid: String, user_id: String, product_id: String) -> Result<HashMap<String, Value>> {
    let order = OrderService::create_order(user_id, product_id).await?;
    let notify_url = config::HTTP_HOST.to_string() + "/api/orders/notify/alipay";
    let client = alipay::new_pay_client(appid.as_str(), Some(notify_url.as_str()))?;

    let mut biz = TradeCreateBiz::new();
    biz.set_buyer_id(order.openid.as_str());
    biz.set_out_trade_no(order.code.as_str());
    biz.set_total_amount((order.amount as f32 / 100 as f32).to_string().as_str());
    biz.set_subject("支付意向金");
    let ret = client.trade_create(&biz)?;

    let mut data: HashMap<String, Value> = HashMap::new();

    if let Some(trade_no) = ret.response.trade_no {
        data.insert(String::from("tradeNo"), Value::String(trade_no));
    }
    else {
        log::error!("创建支付宝订单失败");
        return Err(CustomError::build_dyn("创建订单失败"))
    }
    Ok(data)
}

pub async fn order_payed(out_trade_no: String, trade_no: String, trade_status: String, total_amount: String, buyer_email: String, sign: String) -> Result<String> {
    
    const SUCCESS:&str = "success";
    const FAIL:&str = "fail";

    if trade_status == "WAIT_BUYER_PAY" {
        return Ok(SUCCESS.to_string());
    }
    else if trade_status == "TRADE_SUCCESS" || trade_status == "TRADE_FINISHED" {
        let order_ret = OrderService::find_order_by_code(out_trade_no.clone()).await?;

        if let Some(order) = order_ret {
            let status = OrderStatus::try_from(order.status);
            match status {
                Ok(OrderStatus::Processing) => {
                    return Ok(SUCCESS.to_string());
                }
                Ok(OrderStatus::Completed) => {
                    return Ok(SUCCESS.to_string());
                }
                Ok(OrderStatus::Waiting) => {
                    let money =  total_amount.parse::<i32>()? * 100;
                    if order.amount != money {
                        log::error!("支付金额不正确，订单号={}, 金额={}", out_trade_no.clone(), total_amount);
                        return Ok(FAIL.to_string());
                    }

                    let ret = OrderService::update_pay_success(order.id.unwrap().to_string(),OrderStatus::Processing as i32, trade_no).await?;

                    if ret {
                        return Ok(SUCCESS.to_string());
                    }
                    else {
                        return Ok(FAIL.to_string());
                    }
                },
                Ok(_) => {
                    log::error!("支付订单不是待支付状态，订单号={}", out_trade_no.clone());
                    return Ok(FAIL.to_string());
                },
                Err(e) => {
                    log::error!("{}", e);
                    return Err(CustomError::build_dyn("订单状态异常"));
                },
            }
        }
        else {
            return Ok(FAIL.to_string());
        }
    }
    else {

    }

    Ok("".to_string())

}