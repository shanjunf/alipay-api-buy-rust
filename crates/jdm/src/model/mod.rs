use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use mongodb::{Client, Collection, options::ClientOptions, bson::{doc} };
use crate::config:: { MONGO_URL, MONGO_DB_NAME };
use crate::common::{ Result, CustomError};

pub mod ret;

mod user;
mod page;
mod product;
mod order;

pub use user::User;
pub use page::Page;
pub use ret::Ret;
pub use product::Product;
pub use order::{Order, OrderStatus};

static CLIENT: tokio::sync::OnceCell<Client> = tokio::sync::OnceCell::const_new();

pub async fn init_mongo_client() -> &'static Client {
    CLIENT.get_or_init(|| async {
        let mut client_options = ClientOptions::parse(MONGO_URL).await.unwrap();
        client_options.app_name = Some("My App".to_string());
        let client = Client::with_options(client_options).unwrap();
        client
    }).await
}

pub trait Operator<'a, T> 
where T: Serialize + DeserializeOwned + Unpin + Send + Sync
{
    fn model_name() -> &'a str;
    fn collection() -> Result<Collection<T>> {
        let client = CLIENT.get();
        if let Some(c) = client {
            let collection = c.database(MONGO_DB_NAME).collection::<T>(Self::model_name());
            Ok(collection)
        }
        else {
            Err(CustomError::build_dyn("get collection failure"))
        }
    }
}

macro_rules! collection_macro {
    ($model:ident, $collection:expr) => {
        impl <'a> Operator<'a, $model> for $model {
            fn model_name() -> &'a str { $collection }
        }
    }
}

collection_macro!(User, "users");
collection_macro!(Product, "products");
collection_macro!(Order, "orders");