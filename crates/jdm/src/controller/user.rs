use std::collections::HashMap;
use std::rc::Rc;

use actix_http::header::HeaderValue;
use actix_web::{web, get, put, post, Scope, Responder, HttpRequest, HttpResponse, http::header};
use crate::common::{ CustomError, get_header_value};
use crate::service::{ UserService, ProductService };
use serde::{Deserialize};
use serde_json::{self, json, Value};
use crate::model::{ User, Page, ret, Ret};
use alipay_sdk_rust::biz::{self, SystemOAuthTokenBiz};
use alipay_sdk_rust::pay::{PayClient, Payer};
use crate::middleware::Auth;
use crate::config;
use base64::{ decode };
use std::iter::repeat;
use crate::lib::{ alipay, util };
use crate::rest;

pub const RESOURCE: &str = "/users";

fn get_props() -> HashMap<String, Value> {

    HashMap::new()
}

#[derive(Deserialize, Debug)]
pub struct QueryInfo {
    email: Option<String>,
    code: Option<String>,
    encrypted_data: Option<String>,
    mobile: Option<String>
}



#[post("/login")]
pub async fn login(body: web::Form<QueryInfo>, req: HttpRequest) -> impl Responder {
    type DataType = String;
    let appid = get_header_value(&req, "appid");
    let platform = get_header_value(&req, "platform1");

    if appid.is_none() {
        return ret::build_failure::<DataType>().set_msg("appid不能为空值").to_ok_responder();
    }
    else if body.code.is_none() {
        return ret::build_failure::<DataType>().set_msg("授权码code不能为空").to_ok_responder();
    }
    else {
        let appid = appid.unwrap_or("");
        let platform = platform.unwrap_or("alipay");

        let result = rest::user::login(appid.to_string(), platform.to_string(), body.code.as_ref().unwrap().to_string()).await;

        match result {
            Ok(ret) => {
                return ret::build_data(Some(ret)).to_ok_responder();
            },
            Err(err) => {
                log::error!("{:?}", err);
                if err.as_ref().is::<CustomError>() {
                    return ret::build_failure::<DataType>().set_msg(err.to_string().as_str()).to_ok_responder();
                }
                else {
                    return ret::build_failure::<DataType>().set_msg("系统异常").to_ok_responder();
                }
            } 
        }
    }
}

#[post("/decrypt")]
pub async fn decrypt_data_mobile(form: web::Form<QueryInfo>) -> impl Responder {
    if let Some(encrypted_data) = form.encrypted_data.as_ref() {
        // let data = rest::user::decrypt_data(encrypted_data.to_string());
        // match data {
        //     // Ok(x) => serde_json::to(x)
        // }
        
    }
    else {
        return ret::build_failure::<String>().set_msg("加密数据不能为空").to_ok_responder();
    }
    return ret::build_failure::<String>().set_msg("加密数据不能为空").to_ok_responder();

}

#[put("")]
pub async fn update_user(body: web::Form<QueryInfo>, req: HttpRequest) -> HttpResponse {
    let token = get_header_value(&req, "token");
    if let Some(user_id) = token {
        if let Some(mobile) = body.mobile.as_ref() {
            if mobile.len() < 11 {
                return ret::build_failure::<String>().set_msg("手机号格式不正确").to_ok_responder();
            }
            let ret = rest::user::update_mobile(user_id.to_string(), mobile.to_string()).await;
            match ret {
                Ok(x) => {log::info!("update user {user_id} result is {x}")},
                Err(e) => log::error!("{}", e)
            }
            return ret::build_success::<String>().to_ok_responder();
        }
        else {
            return ret::build_failure::<String>().set_msg("手机号不能为空").to_ok_responder();
        }
    }
    else {
        return ret::build_failure::<String>().set_msg("无权限").to_ok_responder();
    }
}

#[get("")]
pub async fn get_by_params(query: web::Query<QueryInfo>) -> HttpResponse {
    let mut ret:Ret<Page<User>> = Ret{
        status: true,
        msg: "ok".to_string(),
        data: None
    };

    let mut email = "";

    if let Some(e) = query.email.as_deref() {
        email = e;
    }
    else {
        ret.status = false;
        ret.msg = String::from("required email");
        return HttpResponse::Ok().json(ret);
    }

    let r = UserService::get_page_by_email(email, 1, 20).await;

    let p = match r {
        Ok(page) => page,
        Err(err) =>  {
            ret.status = false;
            ret.msg = err.to_string();
            return HttpResponse::Ok().json(ret);
        }
    };

    ret.data = Some(p);

    // let json = serde_json::json!(ret);
    println!("query={:?}", query);
    HttpResponse::Ok().json(ret)
}

// pub fn load_services(scope: Scope) -> Scope {
//     scope.service(login)
//         .service(decrypt_data_mobile)
//         .service(
//             web::scope(RESOURCE)
//                 .service(get_by_params)
//                 .service(update_user)
//             )
// }

pub trait UserScope {
    fn load_user_services(self) -> Scope;
}

impl UserScope for Scope {
    fn load_user_services(self) -> Scope {
        self.service(login)
        .service(decrypt_data_mobile)
        .service(
            web::scope(RESOURCE)
                .service(get_by_params)
                .service(update_user)
            )
    }
}
