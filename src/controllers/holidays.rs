use rocket::get;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json;
use rocket::serde::json::Json;

use diesel::PgConnection;
use diesel::SelectableHelper;
use diesel::RunQueryDsl;
use diesel::QueryResult;
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::query_dsl::QueryDsl;
use diesel::insert_into;
use diesel::delete;

use uuid::Uuid;

use crate::connections::db_connection;
use crate::models::holiday::Holiday;
use crate::models::holiday::NewHoliday;
use crate::models::holiday::ExternalHoliday;
use crate::schema::holidays;

use crate::controllers::to_resp;

// Define a function to search for a row by UUID
fn find_row_by_uuid(uuid: Uuid, conn: &mut PgConnection) -> QueryResult<Option<ExternalHoliday>> {
    return holidays::dsl::holidays
        .filter(holidays::dsl::uuid.eq(uuid))
        .select(ExternalHoliday::as_select())
        .first(conn)
        .optional();
}

#[get("/holidays")]
pub fn index() -> Json<Vec<Holiday>> {
    let conn = &mut db_connection();
    let results = holidays::dsl::holidays
        .select(Holiday::as_select())
        .load(conn)
        .expect("Error loading holidays");
    return Json(results);
}

#[get("/holidays/<holiday_uuid>")]
pub fn show(holiday_uuid: &str) -> Custom<String> {
    let conn = &mut db_connection();

    match Uuid::parse_str(holiday_uuid) {
        Ok(x) => {
            match find_row_by_uuid(x, conn) {
                Ok(Some(row)) => Custom(Status::Ok, json::to_string(&row).unwrap()),
                Ok(None) => Custom(Status::NotFound, to_resp(format!("Holiday {} not found!", holiday_uuid))),
                Err(x) => Custom(Status::InternalServerError, to_resp(format!("Internal error {}",x)))
            }
        },
        Err(x) => Custom(Status::UnprocessableEntity, to_resp(format!("uuid {} wrong format!", x)))
    }
}


#[derive(Responder)]
#[response(status = 201, content_type = "json")]
pub struct CreatedJson(Json<ExternalHoliday>);

#[post("/holidays", format="json", data = "<new_holiday>")]
pub fn create(new_holiday: Json<NewHoliday<'_>>) -> CreatedJson {
    let conn = &mut db_connection();
    let result = insert_into(holidays::dsl::holidays)
        .values(&*new_holiday)
        .returning(ExternalHoliday::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    return CreatedJson(Json(result));
}

#[delete("/holidays/<holiday_uuid>")]
pub fn destroy(holiday_uuid: &str) -> Custom<String> {
    let conn = &mut db_connection();

    match Uuid::parse_str(holiday_uuid) {
        Ok(x) => {
            match find_row_by_uuid(x, conn) {
                Ok(Some(_record)) => {
                    delete(holidays::dsl::holidays)
                        .filter(holidays::dsl::uuid.eq(x))
                        .execute(conn)
                        .expect("Error loading holidays");
                    Custom(Status::NoContent, "".to_string())
                },
                Ok(None) => Custom(Status::NotFound, to_resp(format!("Currency {} not found!", holiday_uuid))),
                Err(x) => Custom(Status::InternalServerError, to_resp(format!("Internal error {}",x)))
            }
        },
        Err(x) => Custom(Status::UnprocessableEntity, to_resp(format!("uuid {} wrong format!", x)))
    }
}

