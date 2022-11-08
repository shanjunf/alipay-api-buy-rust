use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use mongodb::{bson::{doc,oid::ObjectId} };
use chrono::{Utc, DateTime} ;
use mongodb::bson::serde_helpers::chrono_datetime_as_bson_datetime;
use crate::common::{serialize_object_id};
use num_enum::TryFromPrimitive;

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    #[serde(rename = "_id", skip_serializing_if="Option::is_none", serialize_with = "serialize_object_id")]
    pub id: Option<ObjectId>,
    pub code: String,
    pub user_id: String,
    pub mobile: Option<String>,
    pub appid: String,
    pub openid: String,
    pub address: Option<String>,
    pub product_id: String,
    pub product_title: String,
    pub name: Option<String>,
    pub amount: i32,
    pub remain_amount: Option<i32>,
    pub out_trade_no: Option<String>,
    pub buyer_email: Option<String>,
    pub status: i8,
    // #[serde(deserialize_with="deserialize", serialize_with = "datetime_format_localtime")]
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub created: DateTime<Utc>,
    // #[serde(deserialize_with="deserialize", serialize_with = "datetime_format_localtime")]
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub updated: DateTime<Utc>,
}

#[derive(TryFromPrimitive)]
#[repr(i8)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderStatus {
    Deleted = -1,
    Waiting = 0,
    Processing = 1,
    Completed = 2,
    ReFund = 3
}

impl Order {

}
