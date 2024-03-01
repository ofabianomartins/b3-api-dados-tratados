use rocket::get;
use rocket::response::status::NoContent;
use rocket::serde::json::Json;

use diesel::SelectableHelper;
use diesel::RunQueryDsl;
use diesel::query_dsl::QueryDsl;
use diesel::insert_into;
use diesel::delete;

use crate::establish_connection;
use crate::models::Quote;
use crate::models::NewQuote;
use crate::schema::quotes::dsl::*;

#[get("/quotes")]
pub fn index() -> Json<Vec<Quote>> {
    let conn = &mut establish_connection();
    let results = tickers
        .select(Ticker::as_select())
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

#[post("/quotes", format="json", data = "<new_quote>")]
pub async fn create(new_quote: Json<NewQuote<'_>>) -> CreatedJson {
    let conn = &mut establish_connection();
    let result = insert_into(quotes)
        .values(&*new_quote)
        .returning(Quote::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    return CreatedJson(Json(result));
}

