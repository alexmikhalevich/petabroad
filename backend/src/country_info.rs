use actix_web::{get, web, HttpRequest, HttpResponse};
use regex::Regex;

use crate::database::MongoProxy;

#[get("/country/{id}")]
pub async fn get_country_info(req: HttpRequest, h_db: web::Data<MongoProxy>) -> HttpResponse {
    let id = req
        .match_info()
        .get("id")
        .expect("Unable to get `id` from URI");

    println!("Got /country request with id {}", id.clone());

    let re = Regex::new(r"^[A-Z]{2,4}$").unwrap();
    if !re.is_match(id.clone()) {
        return HttpResponse::BadRequest().body("Fuck you");
    }

    match h_db.into_inner().fetch_country_info_json(id).await {
        Some(country_json) => {
            println!("Fetched country: {}", country_json);
            HttpResponse::Ok().body(country_json)
        },
        None => HttpResponse::NotFound().body("No data for this country"),
    }
}
