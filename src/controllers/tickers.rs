
use rocket::get;
use rocket::response::status::NoContent;
use rocket::serde::json::Json;

use diesel::SelectableHelper;
use diesel::RunQueryDsl;
use diesel::query_dsl::QueryDsl;
use diesel::insert_into;
use diesel::delete;

use crate::connections::establish_connection;
use crate::models::Ticker;
use crate::models::NewTicker;
use crate::schema::tickers::dsl::*;

#[get("/tickers")]
pub fn index() -> Json<Vec<Ticker>> {
    let conn = &mut establish_connection();
    let results = tickers
        .select(Ticker::as_select())
        .load(conn)
        .expect("Error loading tickers");
    return Json(results);
}

#[delete("/tickers/<ticker_id>")]
pub fn destroy(ticker_id: i32) -> NoContent {
    let conn = &mut establish_connection();
    delete(tickers.find(ticker_id))
        .execute(conn)
        .expect("Error loading tickers");
    return NoContent;
}

#[derive(Responder)]
#[response(status = 201, content_type = "json")]
pub struct CreatedJson(Json<Ticker>);

#[post("/tickers", format="json", data = "<new_ticker>")]
pub async fn create(new_ticker: Json<NewTicker<'_>>) -> CreatedJson {
    let conn = &mut establish_connection();
    let result = insert_into(tickers)
        .values(&*new_ticker)
        .returning(Ticker::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    return CreatedJson(Json(result));
}

