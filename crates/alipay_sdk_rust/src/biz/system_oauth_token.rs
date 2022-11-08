#![allow(unused)]
use super::{BizContenter, BizObject};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::io::Result;
use std::io::{Error, ErrorKind};

#[derive(Serialize, Deserialize, Default)]
pub struct SystemOAuthTokenBiz(BizObject);

impl BizContenter for SystemOAuthTokenBiz {
    fn method(&self) -> String {
        "alipay.system.oauth.token".to_string()
    }
    // 设置可选字段方法
    fn set(&mut self, key: &str, value: &str) {
        self.0.insert(key.to_string(), value.to_string());
    }

    fn is_biz_content(&self) -> bool {
        false
    }
}
// 以下是设置必选字段方法
impl SystemOAuthTokenBiz {
    pub fn new() -> Self {
        Self(BizObject::new())
    }

    pub fn set_grant_type(&mut self, value: &str) {
        self.set("grant_type", value);
    }

    pub fn set_code(&mut self, value: &str) {
        self.set("code", value);
    }

    pub fn set_refresh_token(&mut self, value: &str) {
        self.set("refresh_token", value);
    }
}