use rocket::get;
use rocket::serde::json::Json;

use diesel::SelectableHelper;
use diesel::RunQueryDsl;
use diesel::query_dsl::QueryDsl;
// use diesel::insert_into;

use crate::establish_connection;
use crate::models::Holiday;
// use crate::models::NewHoliday;
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

//#[post("/holidays", format="json", data = "<new_holiday>")]
//pub async fn create(new_holiday: Json<NewHoliday<'_>>) -> Json<Holiday> {
//    let conn = &mut establish_connection();
//    let result = insert_into(holidays)
//        .values(&*new_holiday)
//        .returning(Holiday::as_returning())
//        .get_result(conn)
//        .expect("Failed to insert sample data into the database");
//
//    return Json(result);
//}

