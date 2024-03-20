use rocket::get;
use rocket::response::status::NoContent;
use rocket::serde::json::Json;

use diesel::SelectableHelper;
use diesel::RunQueryDsl;
use diesel::query_dsl::QueryDsl;
use diesel::delete;

use redis::Commands;

use chrono::Utc;

use crate::connections::db_connection;
use crate::connections::redis_connection;
use crate::models::Quote;
use crate::schema::quotes::dsl::*;

#[get("/quotes")]
pub fn index() -> Json<Vec<Quote>> {
    let conn = &mut db_connection();
    let results = quotes
        .select(Quote::as_select())
        .load(conn)
        .expect("Error loading tickers");
    return Json(results);
}

#[delete("/quotes/<quote_id>")]
pub fn destroy(quote_id: i32) -> NoContent {
    let conn = &mut db_connection();
    delete(quotes.find(quote_id))
        .execute(conn)
        .expect("Error loading quotes");
    return NoContent;
}

#[derive(Responder)]
#[response(status = 201, content_type = "json")]
pub struct CreatedJson(Json<Quote>);

#[post("/quotes", format="json", data = "<quote_params>")]
pub async fn create(quote_params: &str) -> &'static str {
    let conn = &mut redis_connection();

    let _result: i32 = conn.zadd(
        "quote_queue", 
        quote_params.replace("\n", "").replace(" ", ""),
        Utc::now().timestamp()
    ).expect("ZADD failed!");

    return "Send to be process!"
}

