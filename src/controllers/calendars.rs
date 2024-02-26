use rocket::get;
use rocket::serde::json::Json;

use diesel::SelectableHelper;
use diesel::RunQueryDsl;
use diesel::query_dsl::QueryDsl;

use crate::establish_connection;
use crate::models::Calendar;
use crate::schema::calendars::dsl::*;

#[get("/")]
pub fn index() -> Json<Vec<Calendar>> {
    let connection = &mut establish_connection();
    let results = calendars
        .select(Calendar::as_select())
        .load(connection)
        .expect("Error loading calendars");
    return Json(results)
}

