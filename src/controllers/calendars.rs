use rocket::get;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::serde::json;

use diesel::SelectableHelper;
use diesel::RunQueryDsl;
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::query_dsl::QueryDsl;
use diesel::PgConnection;
use diesel::QueryResult;
use diesel::insert_into;
use diesel::update;
use diesel::delete;

use crate::connections::db_connection;
use crate::models::calendar::Calendar;
use crate::models::calendar::NewCalendar;
use crate::schema::calendars;
use crate::controllers::to_resp;

#[get("/calendars")]
pub fn index() -> Json<Vec<Calendar>> {
    let conn = &mut db_connection();
    let results = calendars::dsl::calendars
        .select(Calendar::as_select())
        .load(conn)
        .expect("Error loading calendars");
    return Json(results);
}

// Define a function to search for a row by UUID
fn find_row(id: i32, conn: &mut PgConnection) -> QueryResult<Option<Calendar>> {
    return calendars::dsl::calendars
        .find(id)
        .select(Calendar::as_select())
        .first(conn)
        .optional();
}

#[get("/calendars/<id>")]
pub fn show(id: i32) -> Custom<String> {
    let conn = &mut db_connection();

    match find_row(id, conn) {
        Ok(Some(row)) => Custom(Status::Ok, json::to_string(&row).unwrap()),
        Ok(None) => Custom(Status::NotFound, to_resp(format!("Calendar {} not found!", id))),
        Err(x) => Custom(Status::InternalServerError, to_resp(format!("Internal error {}",x)))
    }
}

#[derive(Responder)]
#[response(status = 201, content_type = "json")]
pub struct CreatedJson(Json<Calendar>);

#[post("/calendars", format="json", data = "<new_calendar>")]
pub async fn create(new_calendar: Json<NewCalendar<'_>>) -> CreatedJson {
    let conn = &mut db_connection();
    let result = insert_into(calendars::dsl::calendars)
        .values(&*new_calendar)
        .returning(Calendar::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    return CreatedJson(Json(result));
}

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
pub struct UpdatedJson(Json<Calendar>);

#[put("/calendars/<id>", format="json", data="<calendar>")]
pub fn update_action(id: i32, calendar: Json<NewCalendar<'_>>) -> UpdatedJson {
    let conn = &mut db_connection();
    update(calendars::dsl::calendars.find(id))
        .set((
            calendars::dsl::name.eq(calendar.name),
            calendars::dsl::code.eq(calendar.code)
        ))
        .returning(Calendar::as_returning())
        .execute(conn)
        .expect("Error loading calendars");

    let result = calendars::dsl::calendars
        .find(id)
        .select(Calendar::as_select())
        .first(conn)
        .expect("Error loading calendars");
    return UpdatedJson(Json(result));
}

#[delete("/calendars/<id>")]
pub fn destroy(id: i32) -> Custom<String> {
    let conn = &mut db_connection();

    match find_row(id, conn) {
        Ok(Some(row)) => {
            delete(calendars::dsl::calendars.find(id))
                .execute(conn)
                .expect("Error loading calendars");
            Custom(Status::NoContent, json::to_string(&row).unwrap())
        },
        Ok(None) => Custom(Status::NotFound, to_resp(format!("Calendar {} not found!", id))),
        Err(x) => Custom(Status::InternalServerError, to_resp(format!("Internal error {}",x)))
    }
}
