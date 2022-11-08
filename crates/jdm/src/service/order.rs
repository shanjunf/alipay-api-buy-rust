
use std::str::FromStr;

use crate::model::{User, Product, Page, Operator, Order, OrderStatus};
use crate::common::{Result, FutureResult, CustomError};
use chrono::{Utc, FixedOffset};
use futures_util::TryStreamExt;
use mongodb::bson::{ doc, oid };
use mongodb::options::{ FindOptions, FindOneOptions };


pub struct OrderService;

impl OrderService {

    pub fn get_order_page_by_user_id(user_id: String, tab: i32, mut page: i32, mut size: i32) -> FutureResult<Page<Order>> {
        
        Box::pin(async move {
            let mut filter = doc!{"user_id": user_id};

            match tab {
                1 => {
                    filter.insert("status", OrderStatus::Waiting as i32);
                },
                2 => {
                    filter.insert("status", OrderStatus::Processing as i32);
                },
                3 => {
                    filter.insert("status", doc!{"$in": [OrderStatus::Processing as i32, OrderStatus::Completed as i32, OrderStatus::ReFund as i32]});
                }
                _ => {}
            }
            
            if page <= 0 {
                page = 1;
            }
            if size <= 0 {
                size = 20;
            }
            let skip = (page - 1) * size;
            let options = FindOptions::builder().sort(doc!{ "time": -1 }).skip(skip as u64).limit(size as i64).build();

            
            let f1 = filter.clone();
            let orders = Order::collection()?;
            let mut cursor = orders.find(f1, options).await?;
            let mut items: Vec<Order> = vec![];
            while let Some(r) = cursor.try_next().await? {
                items.push(r);
            }

            let total = orders.count_documents(filter, None).await?;

            let page = Page::new(items, page as i64, size as u64, total);

            Ok(page)
        })
        
    }

    pub fn update_pay_success(id: String, status: i32, out_trade_no: String) -> FutureResult<bool> {
        Box::pin(async move {
            let order = Order::collection()?.update_one(doc!{"_id": oid::ObjectId::from_str(id.as_str())?}, doc!{"status": status, "out_trade_no": out_trade_no}, None).await?;
            if order.modified_count == 1 {
                return Ok(true);
            }

            Ok(false)
        })
    }

    pub fn find_order_by_code(code: String) -> FutureResult<Option<Order>> {
        Box::pin(async move {
            let order = Order::collection()?.find_one(doc!{"code": code}, None).await?;

            Ok(order)
        })
    }
    pub fn create_order(user_id: String, product_id: String) -> FutureResult<Order> {
        Box::pin(async move {
            let date = Utc::now();
            let local = date.with_timezone(&FixedOffset::east(8*3600));
            let code = local.format("%Y%m%d%H%M%S%6f").to_string();
    
            let users = User::collection()?;
    
            let user_ret = users.find_one(doc!{"_id": oid::ObjectId::from_str(user_id.as_str())?}, None).await?;

            match user_ret {
                Some(user) => {
                    
                    let products = Product::collection()?;
                    let product_ret = products.find_one(doc!{"_id": oid::ObjectId::from_str(product_id.as_str())?}, None).await?;

                    match product_ret {
                        Some(product) => {
                            let mut order = Order{
                                id: None,
                                code,
                                user_id,
                                mobile: user.mobile,
                                openid: user.openid,
                                appid: user.appid,
                                name: user.name,
                                address: user.address,
                                product_id,
                                product_title: product.title,
                                amount: product.deposit,
                                out_trade_no: None,
                                buyer_email: None,
                                remain_amount: Some(product.balance),
                                status: OrderStatus::Waiting as i8,
                                created: date,
                                updated: date
                            };

                            let order_insert_ret = Order::collection()?.insert_one(&order, None).await?;

                            if let Some(id) = order_insert_ret.inserted_id.as_object_id() {
                                order.id = Some(id);
                                return Ok(order);
                            }
                            else {
                                return Err(CustomError::build_dyn("创建订单失败"));
                            }
                        },
                        None => return Err(CustomError::build_dyn("商品不存在")),
                    }
                },
                None => Err(CustomError::build_dyn("用户不存在")),
            }
        })
    }
}