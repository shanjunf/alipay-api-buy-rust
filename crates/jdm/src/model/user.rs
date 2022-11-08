use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use mongodb::{bson::{doc,oid::ObjectId} };
use chrono::{Utc, DateTime} ;
use mongodb::bson::serde_helpers::chrono_datetime_as_bson_datetime;
use crate::common::{serialize_object_id};

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if="Option::is_none", serialize_with = "serialize_object_id")]
    pub id: Option<ObjectId>,
    pub mobile: Option<String>,
    pub name: Option<String>,
    pub address: Option<String>,
    pub platform: String,
    pub appid: String,
    pub openid: String, 
    // #[serde(deserialize_with="deserialize", serialize_with = "datetime_format_localtime")]
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub created: DateTime<Utc>,
    // #[serde(deserialize_with="deserialize", serialize_with = "datetime_format_localtime")]
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub updated: DateTime<Utc>,
}

impl User {

}
