use alipay_sdk_rust::cert;
use alipay_sdk_rust::pay::{ PayClient, Payer};
use std::fs;

use std::io::{Result};

use super::util;

const APP_CERT_SN_FILE: &str = "/crates/jdm/src/lib/alipay/rsa/appCertPublicKey.crt";
const ALIPAY_ROOT_CERT_FILE: &str = "/crates/jdm/src/lib/alipay/rsa/alipayRootCert.crt";
const ALIPAY_CERT_PUBLIC_KEY_RSA2_FILE: &str = "/crates/jdm/src/lib/alipay/rsa/alipayCertPublicKey_RSA2.crt";
const APP_PRIVATE_KEY: &str = "/crates/jdm/src/lib/alipay/rsa/appCertPrivateKey";
const APP_ALIPAY_GATEWAY: &str = "https://openapi.alipay.com/gateway.do";

pub fn new_client(appid: &str) -> Result<impl Payer> {
    new_pay_client(appid, None)
}

pub fn new_pay_client(appid: &str, notify_url: Option<&str>) -> Result<impl Payer> {
    let curdir = util::get_project_root()?.to_str().unwrap_or("").to_string();

    log::info!("curdir={}", curdir);

    let app_sert_sn = cert::get_cert_sn(curdir.clone() + APP_CERT_SN_FILE)?;
    let alipay_root_sert = cert::get_root_cert_sn(curdir.clone() + ALIPAY_ROOT_CERT_FILE)?;
    let alipay_public_key = cert::get_public_key_with_path(curdir.clone() + ALIPAY_CERT_PUBLIC_KEY_RSA2_FILE)?;
    let alipay_private_key = fs::read_to_string(curdir.clone() + APP_PRIVATE_KEY)?;
    let mut cb = PayClient::builder();

cb.api_url(APP_ALIPAY_GATEWAY)
.app_id(appid)
.alipay_root_cert_sn(alipay_root_sert.as_str())
.alipay_public_key(alipay_public_key.as_str())
.app_cert_sn(app_sert_sn.as_str())
.charset_utf8()
.format_json()
.private_key(alipay_private_key.as_str())
.public_key(app_sert_sn.as_str())
.sign_type_rsa2()
.version_1_0();

if let Some(url) = notify_url {
    cb.notify_url(url);
}
let client = cb.build()?;
    // client.get_token("aaa");

    Ok(client)
}