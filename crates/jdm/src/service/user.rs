
use std::str::FromStr;

use crate::model::{User, Page, Operator};
use crate::common::{Result, FutureResult};
use chrono::Utc;
use futures_util::TryStreamExt;
use mongodb::bson::{ doc, oid };
use mongodb::options::{ FindOptions, FindOneOptions };

pub struct UserService;

impl UserService {
    pub fn get_user_by_openid(platform: String, appid: String, openid: String) -> FutureResult<Option<User>> {
        Box::pin(async move {
            let filter = doc!{"platform": platform.clone(), "appid": appid.clone(), "openid": openid.clone()};
            let users = User::collection()?;
            let user = users.find_one(filter, None).await?;
            if let None = user {
                return Self::create_user(platform, appid, openid).await;
            }
            else {
                Ok(user)
            }
        })
    }

    pub fn create_user(platform: String, appid: String, openid: String) -> FutureResult<Option<User>> {
        Box::pin(async move {
            let mut user = User{
                id: None,
                mobile: None,
                name: None,
                address: None,
                platform: platform.clone(),
                appid: appid.clone(),
                openid: openid.clone(),
                created: Utc::now(),
                updated: Utc::now()
            };

            let users = User::collection()?;
            let r = users.insert_one(&user, None).await?;
            if let Some(object_id) = r.inserted_id.as_object_id() {
                user.id = Some(object_id);
                return Ok(Some(user))
            }
            else {
                Ok(None)
            }
        })
    }

    pub fn update_mobile(id: String, mobile: String) -> FutureResult<bool> {
        Box::pin(async move {
            let users = User::collection()?;
            
            let query = doc!{"_id": oid::ObjectId::from_str(id.as_str())?};
            let update = doc!{"$set": {"mobile": mobile, "updated": Utc::now()}};

            let r = users.update_one(query, update, None).await?;
            if r.modified_count > 0 {
                return Ok(true)
            }
            else {
                Ok(false)
            }
        })
    }

    pub fn get_page_by_email(email: &str, mut page: i64, size: u64) -> FutureResult<Page<User>> {
        let filter = doc!{"email": email};
        if page <= 0 {
            page = 1;
        }
        let skip = (page as u64 - 1) * size;
        let options = FindOptions::builder().sort(doc!{ "time": -1 }).skip(skip).limit(20).build();

        Box::pin(async move {
            let f1 = filter.clone();
            let users = User::collection()?;
            let mut cursor = users.find(f1, options).await?;
            let mut items: Vec<User> = vec![];
            while let Some(r) = cursor.try_next().await? {
                items.push(r);
            }

            let total = users.count_documents(filter, None).await?;

            let page = Page::new(items, page, size, total);

            Ok(page)
        })
    }
/*
    fn fff() {
        // let data = "f84qR9IpSvJhTyma5rp6525IylUoGf7U/38xk4FzTcRVskAsAcfMg2XM++t6znNikhS8WBPIly8TpgKhGr+zMlmOM4XhRz/urjMx4VHsKYNk3rxHf+YiuOJC3T2s1O49WynJREdxm4NPJdLv2OuDKgtYPaL8OXJFnXm71C7TGFzwWxpktjV73zVgfybj/KmGkkxxlFahxypxRvnXXJUWdg13FIsQPSUt1CDT0EaQ80PohwFhOmLomHWLeGKOspt3nEgCYtLDNbLbzxfb9b5eEQlKXmMa8ukSCEFKchPf0sWolUa1hMd+enYVWmFVxGxX/YEDsM0EZXJV3XJzNy7byXWoPa/zx68iMQ6ogXd9+RY=";
        let mut data: Option<HashMap<String, Value>> = Some(HashMap::new());
        if let Some(en_data) = query.encrypted_data.as_ref() {
            // let decrypt_data = crypt::aes::decrypt_data(en_data.as_str(), config::ENCRYPT_TYPE, config::ENCRYPT_KEY);
            // data.as_mut().unwrap().insert("decrypted_data".to_string(), Value::String(decrypt_data.to_string()));

            let mut iv: Vec<u8> = repeat(0u8).take(16).collect();
            let key = &decode(config::ENCRYPT_KEY).unwrap();
            let rdata = &decode(en_data).unwrap();
            let ret = util::aes256_cbc_decrypt(rdata, key, &iv[..]).unwrap();
            // let pt = crypt::aes::decrypt(&ct, iv);
            println!("解密结果：{:?}", String::from_utf8(ret));

            data.as_mut().unwrap().insert("decrypted_data".to_string(), Value::String(String::from_utf8(ret)));

            match String::from_utf8(ret) {
                Ok(x) => {},
                Err(e) => {
                    println!("error: :?", e)
                }
            }

            if let Ok(ret) = String::from_utf8(ret) {

            }
            else {

            }
            
            Ret::build_success(data).to_ok_responder()
        }
        else {
            return Ret::build_failure(data).set_msg(String::from("encrypted_data不能为空")).to_ok_responder();
        }
    }
     */
}