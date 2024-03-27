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
use crate::models::Calendar;
use crate::models::NewCalendar;
use crate::schema::calendars::dsl::*;

#[get("/calendars")]
pub fn index() -> Json<Vec<Calendar>> {
    let conn = &mut db_connection();
    let results = calendars
        .select(Calendar::as_select())
        .load(conn)
        .expect("Error loading calendars");
    return Json(results);
}

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
pub struct ShowJson(Json<Calendar>);

#[get("/calendars/<calendar_id>")]
pub fn show(calendar_id: i32) -> ShowJson {
    let conn = &mut db_connection();
    let result = calendars
        .find(calendar_id)
        .select(Calendar::as_select())
        .first(conn)
        .expect("Error loading calendars");
    return ShowJson(Json(result));
}

#[derive(Responder)]
#[response(status = 201, content_type = "json")]
pub struct CreatedJson(Json<Calendar>);

#[post("/calendars", format="json", data = "<new_calendar>")]
pub async fn create(new_calendar: Json<NewCalendar<'_>>) -> CreatedJson {
    let conn = &mut db_connection();
    let result = insert_into(calendars)
        .values(&*new_calendar)
        .returning(Calendar::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    return CreatedJson(Json(result));
}

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
pub struct UpdatedJson(Json<Calendar>);

#[put("/calendars/<calendar_id>", format="json", data="<calendar>")]
pub fn update_action(calendar_id: i32, calendar: Json<NewCalendar<'_>>) -> UpdatedJson {
    let conn = &mut db_connection();
    update(calendars.find(calendar_id))
        .set((
            name.eq(calendar.name),
            code.eq(calendar.code)
        ))
        .returning(Calendar::as_returning())
        .execute(conn)
        .expect("Error loading calendars");

    let result = calendars
        .find(calendar_id)
        .select(Calendar::as_select())
        .first(conn)
        .expect("Error loading calendars");
    return UpdatedJson(Json(result));
}

#[delete("/calendars/<calendar_id>")]
pub fn destroy(calendar_id: i32) -> NoContent {
    let conn = &mut db_connection();
    delete(calendars.find(calendar_id))
        .execute(conn)
        .expect("Error loading calendars");
    return NoContent;
}
