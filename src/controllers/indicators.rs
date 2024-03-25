use rocket::get;
use rocket::response::status::NoContent;
use rocket::serde::json::Json;

use diesel::SelectableHelper;
use diesel::RunQueryDsl;
use diesel::query_dsl::QueryDsl;
use diesel::insert_into;
use diesel::delete;

use crate::connections::db_connection;
use crate::models::Indicator;
use crate::models::NewIndicator;
use crate::schema::indicators::dsl::*;

#[get("/indicators")]
pub fn index() -> Json<Vec<Indicator>> {
    let conn = &mut db_connection();
    let results = indicators
        .select(Indicator::as_select())
        .load(conn)
        .expect("Error loading indicators");
    return Json(results);
}

#[delete("/indicators/<indicator_id>")]
pub fn destroy(indicator_id: i32) -> NoContent {
    let conn = &mut db_connection();
    delete(indicators.find(indicator_id))
        .execute(conn)
        .expect("Error loading indicators");
    return NoContent;
}

#[derive(Responder)]
#[response(status = 201, content_type = "json")]
pub struct CreatedJson(Json<Indicator>);

#[post("/indicators", format="json", data = "<new_indicator>")]
pub async fn create(new_indicator: Json<NewIndicator<'_>>) -> CreatedJson {
    let conn = &mut db_connection();
    let result = insert_into(indicators)
        .values(&*new_indicator)
        .returning(Indicator::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    return CreatedJson(Json(result));
}

