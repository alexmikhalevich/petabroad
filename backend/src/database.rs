use mongodb::{bson::doc, bson::Document, error::Result, options::ClientOptions, Client, Collection};
use std::env;

const MONGODB_USER_ENV: &'static str = "MONGODB_USER";
const MONGODB_PASSWORD_ENV: &'static str = "MONGODB_PASSWORD";
const MONGODB_ENDPOINT_ENV: &'static str = "MONGODB_ENDPOINT";
const MONGODB_DB_NAME_ENV: &'static str = "MONGODB_DB_NAME";
const MONGODB_APP_NAME: &'static str = "petabroad-backend";

#[derive(Clone, Debug)]
pub struct MongoProxy {
    pub client: Client,
    pub db_name: String,
}

impl MongoProxy {
    pub async fn init() -> Result<Self> {
        let db_name = env::var(MONGODB_DB_NAME_ENV).expect(&format!(
            "You need to set `{}` environment variable",
            MONGODB_DB_NAME_ENV
        ));

        let user = env::var(MONGODB_USER_ENV).expect(&format!(
            "You need to set `{}` environment variable",
            MONGODB_USER_ENV
        ));
        let password = env::var(MONGODB_PASSWORD_ENV).expect(&format!(
            "You need to set `{}` environment variable",
            MONGODB_PASSWORD_ENV
        ));
        let endpoint = env::var(MONGODB_ENDPOINT_ENV).expect(&format!(
            "You need to set `{}` environment variable",
            MONGODB_ENDPOINT_ENV
        ));

        let mut client_options = ClientOptions::parse(&format!("mongodb://{}", endpoint)).await?;
        client_options.app_name = Some(MONGODB_APP_NAME.to_string());
        Ok(Self {
            client: Client::with_options(client_options)?,
            db_name: db_name,
        })
    }

    fn get_collection(&self, collection_name: &str) -> Collection<Document> {
        self.client
            .database(&self.db_name.clone())
            .collection(collection_name)
    }

    pub async fn fetch_country_info_json(&self, id: &str) -> Option<String> {
        let country_doc = self.get_collection("countries")
            .find_one(doc! {"id": id}, None)
            .await
            .expect("Error occured during fetching country info");
        if country_doc == None {
            return None;
        } else {
            return match serde_json::to_string(&country_doc.unwrap()) {
                Ok(val) => Some(val),
                Err(e) => {
                    println!("Unable to serialize country {}: {}", id, e);
                    None
                },
            };
        }
    }
}
