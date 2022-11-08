use serde::{Deserialize, Serialize};
use mongodb::{bson::{doc,oid::ObjectId} };
use chrono::{Utc, DateTime} ;
use mongodb::bson::serde_helpers::chrono_datetime_as_bson_datetime;
use crate::common::{serialize_object_id};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    #[serde(rename = "_id", skip_serializing_if="Option::is_none", serialize_with = "serialize_object_id")]
    pub id: Option<ObjectId>,
    pub image: String,
    pub title: String,
    pub deposit: i32, 
    pub balance: i32, 
    pub desc: String,
    // #[serde(deserialize_with="deserialize", serialize_with = "datetime_format_localtime")]
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub created: DateTime<Utc>,
    // #[serde(deserialize_with="deserialize", serialize_with = "datetime_format_localtime")]
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub updated: DateTime<Utc>
}

impl Product {

}
