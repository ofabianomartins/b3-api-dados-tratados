
use rocket::get;
use rocket::response::status::NoContent;
use rocket::serde::json::Json;

use diesel::SelectableHelper;
use diesel::RunQueryDsl;
use diesel::ExpressionMethods;
use diesel::query_dsl::QueryDsl;
use diesel::insert_into;
use diesel::update;
use diesel::delete;

use crate::connections::db_connection;
use crate::models::ticker::Ticker;
use crate::models::ticker::NewTicker;
use crate::schema::tickers;

#[get("/tickers?<symbol>")]
pub fn index(symbol: Option<String>) -> Json<Vec<Ticker>> {
    let conn = &mut db_connection();

    let results: Vec<Ticker>;

    match symbol {
        Some(x) => {
            results = tickers::dsl::tickers
                .filter(tickers::dsl::symbol.eq(x))
                .select(Ticker::as_select())
                .load(conn)
                .expect("Error loading tickers");
        },
        None => {
            results = tickers::dsl::tickers
                .select(Ticker::as_select())
                .load(conn)
                .expect("Error loading tickers");

        }
    }
    return Json(results);
}

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
pub struct UpdatedJson(Json<Ticker>);

#[put("/tickers/<ticker_id>", format="json", data="<ticker>")]
pub fn update_action(ticker_id: i32, ticker: Json<NewTicker<'_>>) -> UpdatedJson {
    let conn = &mut db_connection();
    update(tickers::dsl::tickers.find(ticker_id))
        .set((
            tickers::dsl::symbol.eq(ticker.symbol),
            tickers::dsl::security_type.eq(ticker.security_type),
            tickers::dsl::unit.eq(ticker.unit),
            tickers::dsl::creation_date.eq(ticker.creation_date),
            tickers::dsl::company_id.eq(ticker.company_id),
            tickers::dsl::currency_id.eq(ticker.currency_id),
            tickers::dsl::calendar_id.eq(ticker.calendar_id),
            tickers::dsl::segment_id.eq(ticker.segment_id)
        ))
        .returning(Ticker::as_returning())
        .execute(conn)
        .expect("Error loading tickers");

    let result = tickers::dsl::tickers
        .find(ticker_id)
        .select(Ticker::as_select())
        .first(conn)
        .expect("Error loading tickers");
    return UpdatedJson(Json(result));
}

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
pub struct ShowJson(Json<Ticker>);

#[get("/tickers/<ticker_id>")]
pub fn show(ticker_id: i32) -> ShowJson {
    let conn = &mut db_connection();
    let result = tickers::dsl::tickers
        .find(ticker_id)
        .select(Ticker::as_select())
        .first(conn)
        .expect("Error loading tickers");
    return ShowJson(Json(result));
}

#[derive(Responder)]
#[response(status = 201, content_type = "json")]
pub struct CreatedJson(Json<Ticker>);

#[post("/tickers", format="json", data = "<new_ticker>")]
pub async fn create(new_ticker: Json<NewTicker<'_>>) -> CreatedJson {
    let conn = &mut db_connection();
    let result = insert_into(tickers::dsl::tickers)
        .values(&*new_ticker)
        .returning(Ticker::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    return CreatedJson(Json(result));
}

#[delete("/tickers/<ticker_id>")]
pub fn destroy(ticker_id: i32) -> NoContent {
    let conn = &mut db_connection();
    delete(tickers::dsl::tickers.find(ticker_id))
        .execute(conn)
        .expect("Error loading tickers");
    return NoContent;
}


