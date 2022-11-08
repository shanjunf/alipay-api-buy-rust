use gostd::strings;
use serde::Serialize;
use std::collections::HashMap;

/// 独立请求参数接口 BizContenter
pub trait ParamContenter: Serialize {
    fn method(&self) -> String;
    fn set(&mut self, key: &str, value: &str);
}

pub type ParamObject = HashMap<String, String>;

/// 返回 例如“alipay.trade.create.response”的返回字段key
pub fn get_response_key(param: &impl ParamContenter) -> String {
    let method = param.method() + ".response";
    strings::ReplaceAll(method, ".", "_")
}
