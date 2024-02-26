use rocket::get;
use rocket::serde::json::Json;

use diesel::SelectableHelper;
use diesel::RunQueryDsl;
use diesel::query_dsl::QueryDsl;
use diesel::insert_into;

use crate::establish_connection;
use crate::models::Calendar;
use crate::models::NewCalendar;
use crate::schema::calendars::dsl::*;

#[get("/calendars")]
pub fn index() -> Json<Vec<Calendar>> {
    let conn = &mut establish_connection();
    let results = calendars
        .select(Calendar::as_select())
        .load(conn)
        .expect("Error loading calendars");
    return Json(results);
}

#[post("/calendars", format="json", data = "<new_calendar>")]
pub async fn create(new_calendar: Json<NewCalendar<'_>>) -> Json<Calendar> {
    let conn = &mut establish_connection();
    let result = insert_into(calendars)
        .values(&*new_calendar)
        .returning(Calendar::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    return Json(result);
}

