use rocket::get;
use rocket::response::status::NoContent;
use rocket::serde::json::Json;

use diesel::SelectableHelper;
use diesel::RunQueryDsl;
use diesel::query_dsl::QueryDsl;
use diesel::insert_into;
use diesel::delete;

use crate::connections::establish_connection;
use crate::models::Holiday;
use crate::models::NewHoliday;
use crate::schema::holidays::dsl::*;

#[get("/holidays")]
pub fn index() -> Json<Vec<Holiday>> {
    let conn = &mut establish_connection();
    let results = holidays
        .select(Holiday::as_select())
        .load(conn)
        .expect("Error loading holidays");
    return Json(results);
}

#[delete("/holidays/<holiday_id>")]
pub fn destroy(holiday_id: i32) -> NoContent {
    let conn = &mut establish_connection();
    delete(holidays.find(holiday_id))
        .execute(conn)
        .expect("Error loading holidays");
    return NoContent;
}

#[derive(Responder)]
#[response(status = 201, content_type = "json")]
pub struct CreatedJson(Json<Holiday>);

#[post("/holidays", format="json", data = "<new_holiday>")]
pub fn create(new_holiday: Json<NewHoliday<'_>>) -> CreatedJson {
    let conn = &mut establish_connection();
    let result = insert_into(holidays)
        .values(&*new_holiday)
        .returning(Holiday::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    return CreatedJson(Json(result));
}

