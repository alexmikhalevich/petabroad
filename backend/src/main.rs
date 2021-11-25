mod country_info;
mod database;

use crate::country_info::get_country_info;
use crate::database::MongoProxy;
use actix_web::{App, HttpServer, web};
use std::env;

const SERVER_ADDRESS_ENV: &'static str = "PETABROAD_BACKEND_ADDRESS";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_address = match env::var(SERVER_ADDRESS_ENV) {
        Ok(val) => val,
        Err(e) => panic!(
            "Error: {}. You need to set {} environment variable",
            e, SERVER_ADDRESS_ENV
        ),
    };

    println!("Starting petabroad-backend on {}", server_address.clone());

    let h_db: MongoProxy = MongoProxy::init()
        .await
        .expect("Unable to connect to database");
    HttpServer::new(move || {
        App::new()
            .service(get_country_info)
            .app_data(web::Data::new(h_db.clone()))
    })
    .bind(server_address)?
    .run()
    .await
}
