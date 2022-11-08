use std::collections::HashMap;

use actix_http::header::HeaderValue;
use actix_web::{web, get, put, post, Scope, Responder, HttpRequest, HttpResponse, http::header};
use crate::service::{ UserService, ProductService };
use serde::{Deserialize};
use serde_json::{self, json, Value};
use crate::model::{ User, Page, Ret };
use alipay_sdk_rust::biz::{self, SystemOAuthTokenBiz};
use alipay_sdk_rust::pay::{PayClient, Payer};
use crate::common::{Result, CustomError};
use base64::{ decode };
use std::iter::repeat;
use crate::lib::{ alipay, util };
use crate::config;

pub async fn login(appid: String, platform: String, auth_code: String) -> Result<HashMap<String, Value>> {

    let mut data: HashMap<String, Value> = HashMap::new();

    let client = alipay::new_client(appid.as_str())?;
    let mut biz_content = SystemOAuthTokenBiz::new();
    biz_content.set_code(auth_code.as_str());
    biz_content.set_grant_type("authorization_code");

    let ret = client.system_oauth_token(&biz_content)?;

    let alipay_user_id = ret.response.user_id.as_ref();
    if let Some(open_id) = alipay_user_id {
        let user = UserService::get_user_by_openid(platform.to_string(), appid.to_string(), open_id.to_owned()).await?;
        
        match user {
            Some(x) => {
                let uid = x.id.unwrap().to_string();
                let val = json!({"token": uid});
                data = serde_json::from_value(val).unwrap_or(data);
                let products = ProductService::get_products().await?;

                let product_value = serde_json::to_value(products)?;
                data.insert("products".to_string(), product_value);
                data.insert("tel".to_string(), Value::String("18888888888".to_string()));
                if let Some(mobile) = x.mobile {
                    data.insert("userMobile".to_string(), Value::String(mobile));
                }
            },
            None => {
                return Err(CustomError::build_dyn("登录失败"));
            }
        }
    }

    Ok(data)
}

pub async fn decrypt_data(encrypted_data: String, user_id: String) -> Result<String> {
    let mut iv: Vec<u8> = repeat(0u8).take(16).collect();
    let key = &decode(config::ENCRYPT_KEY)?;
    let data = &decode(encrypted_data)?;
    let ret = util::aes256_cbc_decrypt(data, key, &iv[..])?;

    // let str = String::from_utf8(ret).unwrap(); 
    match String::from_utf8(ret) {
        Ok(x) => {
            // UserService::update_user_mobile()
            let unstr = serde_json::from_str(&format!("\"{}\"", x))?;

            let value = serde_json::to_value(unstr)?;

            let arr = value.as_array();

            match process_decrypt_data(value) {
                Some(m) => {
                    let mobile = "18888888888".to_string();
                    UserService::update_mobile(user_id, mobile).await?;
                    Ok("()".to_string())
                },
                None => Err(CustomError::build_dyn("解密失败"))
            }
        },
        Err(e) => Err(CustomError::build_dyn("解密失败"))
    }
}

pub async fn  update_mobile(user_id: String, mobile: String) -> Result<bool> {
    let ret = UserService::update_mobile(user_id, mobile).await?;
    Ok(ret)
}

fn process_decrypt_data(value: Value) -> Option<String> {
    let map = value.as_object();
    println!("{:?}", map);

    None
}