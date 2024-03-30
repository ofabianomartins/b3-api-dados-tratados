use rocket::get;
use rocket::response::status::NoContent;
use rocket::serde::json::Json;

use diesel::SelectableHelper;
use diesel::RunQueryDsl;
use diesel::query_dsl::QueryDsl;
use diesel::insert_into;
use diesel::delete;

use crate::connections::db_connection;
use crate::models::holiday::Holiday;
use crate::models::holiday::NewHoliday;
use crate::schema::holidays::dsl::*;

#[get("/holidays")]
pub fn index() -> Json<Vec<Holiday>> {
    let conn = &mut db_connection();
    let results = holidays
        .select(Holiday::as_select())
        .load(conn)
        .expect("Error loading holidays");
    return Json(results);
}

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
pub struct ShowJson(Json<Holiday>);

#[get("/holidays/<holiday_id>")]
pub fn show(holiday_id: i32) -> ShowJson {
    let conn = &mut db_connection();
    let result = holidays
        .find(holiday_id)
        .select(Holiday::as_select())
        .first(conn)
        .expect("Error loading holidays");
    return ShowJson(Json(result));
}

#[derive(Responder)]
#[response(status = 201, content_type = "json")]
pub struct CreatedJson(Json<Holiday>);

#[post("/holidays", format="json", data = "<new_holiday>")]
pub fn create(new_holiday: Json<NewHoliday<'_>>) -> CreatedJson {
    let conn = &mut db_connection();
    let result = insert_into(holidays)
        .values(&*new_holiday)
        .returning(Holiday::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    return CreatedJson(Json(result));
}

#[delete("/holidays/<holiday_id>")]
pub fn destroy(holiday_id: i32) -> NoContent {
    let conn = &mut db_connection();
    delete(holidays.find(holiday_id))
        .execute(conn)
        .expect("Error loading holidays");
    return NoContent;
}

