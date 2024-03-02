use rocket::get;
use rocket::response::status::NoContent;
use rocket::serde::json::Json;

use diesel::SelectableHelper;
use diesel::RunQueryDsl;
use diesel::query_dsl::QueryDsl;
//use diesel::insert_into;
use diesel::delete;

use serde::Serialize;
use serde::Deserialize;

use std::fmt::Debug;

use chrono::NaiveDate;
use bigdecimal::BigDecimal;

use crate::connections::establish_connection;
use crate::models::Quote;
// use crate::models::NewQuote;
use crate::schema::quotes::dsl::*;

#[get("/quotes")]
pub fn index() -> Json<Vec<Quote>> {
    let conn = &mut establish_connection();
    let results = quotes
        .select(Quote::as_select())
        .load(conn)
        .expect("Error loading tickers");
    return Json(results);
}

#[delete("/quotes/<quote_id>")]
pub fn destroy(quote_id: i32) -> NoContent {
    let conn = &mut establish_connection();
    delete(quotes.find(quote_id))
        .execute(conn)
        .expect("Error loading quotes");
    return NoContent;
}

#[derive(Responder)]
#[response(status = 201, content_type = "json")]
pub struct CreatedJson(Json<Quote>);

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateParams {
    pub date: NaiveDate,
    pub symbol: String,
    pub close: BigDecimal,
	pub open: Option<BigDecimal>,
	pub high: Option<BigDecimal>,
	pub low: Option<BigDecimal>,
	pub average: Option<BigDecimal>,
	pub ask: Option<BigDecimal>,
	pub bid: Option<BigDecimal>,
	pub adjust: Option<BigDecimal>,
	pub volume: Option<BigDecimal>,
	pub trades: Option<BigDecimal>,
}

//#[post("/quotes", format="json", data = "<quote_params>")]
#[post("/quotes")]
pub async fn create() -> &'static str {
    return "TODO: Implement backgroung insert"
// pub async fn create(quote_params: Json<CreateParams>) -> CreatedJson {
//    let conn = &mut establish_connection();
//    let result = insert_into(quotes)
//        .values(&*new_quote)
//        .returning(Quote::as_returning())
//        .get_result(conn)
//        .expect("Failed to insert sample data into the database");
//
//    return CreatedJson(Json(result));
}

