use diesel::PgConnection;
use rocket::get;
use rocket::response::status::NoContent;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::serde::json;

use diesel::SelectableHelper;
use diesel::RunQueryDsl;
use diesel::ExpressionMethods;
use diesel::QueryResult;
use diesel::OptionalExtension;
use diesel::query_dsl::QueryDsl;
use diesel::insert_into;
use diesel::update;
use diesel::delete;

use uuid::Uuid;

use crate::connections::db_connection;
use crate::models::calendar::Calendar;
use crate::models::calendar::NewCalendar;
use crate::models::calendar::ExternalCalendar;
use crate::schema::calendars;
use crate::controllers::to_resp;

#[get("/calendars")]
pub fn index() -> Json<Vec<ExternalCalendar>> {
    let conn = &mut db_connection();
    let results = calendars::dsl::calendars
        .select(ExternalCalendar::as_select())
        .load(conn)
        .expect("Error loading calendars");
    return Json(results);
}

// Define a function to search for a row by UUID
fn find_row_by_uuid(uuid: Uuid, conn: &mut PgConnection) -> QueryResult<Option<ExternalCalendar>> {
    return calendars::dsl::calendars
        .filter(calendars::dsl::uuid.eq(uuid))
        .select(ExternalCalendar::as_select())
        .first(conn)
        .optional();
}

#[get("/calendars/<calendar_uuid>")]
pub fn show(calendar_uuid: &str) -> Custom<String> {
    let conn = &mut db_connection();

    match Uuid::parse_str(calendar_uuid) {
        Ok(x) => {
            match find_row_by_uuid(x, conn) {
                Ok(Some(row)) => Custom(Status::Ok, json::to_string(&row).unwrap()),
                Ok(None) => Custom(Status::NotFound, to_resp(format!("Calendar {} not found!", calendar_uuid))),
                Err(x) => Custom(Status::InternalServerError, to_resp(format!("Internal error {}",x)))
            }
        },
        Err(x) => Custom(Status::UnprocessableEntity, to_resp(format!("uuid {} wrong format!", x)))
    }
}

#[derive(Responder)]
#[response(status = 201, content_type = "json")]
pub struct CreatedJson(Json<ExternalCalendar>);

#[post("/calendars", format="json", data = "<new_calendar>")]
pub async fn create(new_calendar: Json<NewCalendar<'_>>) -> CreatedJson {
    let conn = &mut db_connection();
    let result = insert_into(calendars::dsl::calendars)
        .values(&*new_calendar)
        .returning(ExternalCalendar::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    return CreatedJson(Json(result));
}

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
pub struct UpdatedJson(Json<ExternalCalendar>);

#[put("/calendars/<calendar_id>", format="json", data="<calendar>")]
pub fn update_action(calendar_id: i32, calendar: Json<NewCalendar<'_>>) -> UpdatedJson {
    let conn = &mut db_connection();
    update(calendars::dsl::calendars.find(calendar_id))
        .set((
            calendars::dsl::name.eq(calendar.name),
            calendars::dsl::code.eq(calendar.code)
        ))
        .returning(Calendar::as_returning())
        .execute(conn)
        .expect("Error loading calendars");

    let result = calendars::dsl::calendars
        .find(calendar_id)
        .select(ExternalCalendar::as_select())
        .first(conn)
        .expect("Error loading calendars");
    return UpdatedJson(Json(result));
}

#[delete("/calendars/<calendar_id>")]
pub fn destroy(calendar_id: i32) -> NoContent {
    let conn = &mut db_connection();
    delete(calendars::dsl::calendars.find(calendar_id))
        .execute(conn)
        .expect("Error loading calendars");
    return NoContent;
}
