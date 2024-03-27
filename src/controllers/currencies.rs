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
use crate::models::Currency;
use crate::models::NewCurrency;
use crate::schema::currencies::dsl::*;

#[get("/currencies")]
pub fn index() -> Json<Vec<Currency>> {
    let conn = &mut db_connection();
    let results = currencies
        .select(Currency::as_select())
        .load(conn)
        .expect("Error loading currencies");
    return Json(results);
}

#[derive(Responder)]
#[response(status = 201, content_type = "json")]
pub struct CreatedJson(Json<Currency>);

#[post("/currencies", format="json", data = "<new_currency>")]
pub async fn create(new_currency: Json<NewCurrency<'_>>) -> CreatedJson {
    let conn = &mut db_connection();
    let result = insert_into(currencies)
        .values(&*new_currency)
        .returning(Currency::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    return CreatedJson(Json(result));
}

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
pub struct ShowJson(Json<Currency>);

#[get("/currencies/<currency_id>")]
pub fn show(currency_id: i32) -> ShowJson {
    let conn = &mut db_connection();
    let result = currencies
        .find(currency_id)
        .select(Currency::as_select())
        .first(conn)
        .expect("Error loading currencies");
    return ShowJson(Json(result));
}

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
pub struct UpdatedJson(Json<Currency>);

#[put("/currencies/<currency_id>", format="json", data="<currency>")]
pub fn update_action(currency_id: i32, currency: Json<NewCurrency<'_>>) -> UpdatedJson {
    let conn = &mut db_connection();
    update(currencies.find(currency_id))
        .set((
            name.eq(currency.name),
            code.eq(currency.code)
        ))
        .returning(Currency::as_returning())
        .execute(conn)
        .expect("Error loading currencies");

    let result = currencies
        .find(currency_id)
        .select(Currency::as_select())
        .first(conn)
        .expect("Error loading currencies");
    return UpdatedJson(Json(result));
}

#[delete("/currencies/<currency_id>")]
pub fn destroy(currency_id: i32) -> NoContent {
    let conn = &mut db_connection();
    delete(currencies.find(currency_id))
        .execute(conn)
        .expect("Error loading currencies");
    return NoContent;
}

