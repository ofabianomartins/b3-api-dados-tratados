use rocket::get;
use rocket::response::status::NoContent;
use rocket::serde::json::Json;

use diesel::SelectableHelper;
use diesel::RunQueryDsl;
use diesel::query_dsl::QueryDsl;
use diesel::insert_into;
use diesel::delete;

use crate::establish_connection;
use crate::models::Currency;
use crate::models::NewCurrency;
use crate::schema::currencies::dsl::*;

#[get("/currencies")]
pub fn index() -> Json<Vec<Currency>> {
    let conn = &mut establish_connection();
    let results = currencies
        .select(Currency::as_select())
        .load(conn)
        .expect("Error loading calendars");
    return Json(results);
}

#[delete("/currencies/<currency_id>")]
pub fn destroy(currency_id: i32) -> NoContent {
    let conn = &mut establish_connection();
    delete(currencies.find(currency_id))
        .execute(conn)
        .expect("Error loading calendars");
    return NoContent;
}

#[derive(Responder)]
#[response(status = 201, content_type = "json")]
pub struct CreatedJson(Json<Currency>);

#[post("/currencies", format="json", data = "<new_currency>")]
pub async fn create(new_currency: Json<NewCurrency<'_>>) -> CreatedJson {
    let conn = &mut establish_connection();
    let result = insert_into(currencies)
        .values(&*new_currency)
        .returning(Currency::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    return CreatedJson(Json(result));
}

