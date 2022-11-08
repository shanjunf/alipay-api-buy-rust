
use crate::model::{Product, Page, Operator};
use crate::common::{Result, FutureResult};
use chrono::Utc;
use futures_util::TryStreamExt;
use mongodb::bson::{ doc };
use mongodb::options::{ FindOptions, FindOneOptions };

pub struct ProductService;

impl ProductService {
    pub fn get_products() -> FutureResult<Vec<Product>> {
        Box::pin(async move {
            let filter = doc!{};
            let products = Product::collection()?;
            let mut cursor = products.find(filter, None).await?;

            let mut items: Vec<Product> = vec![];
            while let Some(r) = cursor.try_next().await? {
                items.push(r);
            }

            Ok(items)
        })
    }
}