use rocket::get;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json;
use rocket::serde::json::Json;

use diesel::PgConnection;
use diesel::SelectableHelper;
use diesel::RunQueryDsl;
use diesel::QueryResult;
use diesel::OptionalExtension;
use diesel::query_dsl::QueryDsl;
use diesel::insert_into;
use diesel::delete;

use crate::connections::db_connection;
use crate::models::holiday::Holiday;
use crate::models::holiday::NewHoliday;
use crate::schema::holidays;

use crate::controllers::to_resp;

// Define a function to search for a row by UUID
fn find_row(id: i32, conn: &mut PgConnection) -> QueryResult<Option<Holiday>> {
    return holidays::dsl::holidays
        .find(id)
        .select(Holiday::as_select())
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

#[get("/holidays/<id>")]
pub fn show(id: i32) -> Custom<String> {
    let conn = &mut db_connection();

    match find_row(id, conn) {
        Ok(Some(row)) => Custom(Status::Ok, json::to_string(&row).unwrap()),
        Ok(None) => Custom(Status::NotFound, to_resp(format!("Holiday {} not found!", id))),
        Err(x) => Custom(Status::InternalServerError, to_resp(format!("Internal error {}",x)))
    }
}


#[derive(Responder)]
#[response(status = 201, content_type = "json")]
pub struct CreatedJson(Json<Holiday>);

#[post("/holidays", format="json", data = "<new_holiday>")]
pub fn create(new_holiday: Json<NewHoliday<'_>>) -> CreatedJson {
    let conn = &mut db_connection();
    let result = insert_into(holidays::dsl::holidays)
        .values(&*new_holiday)
        .returning(Holiday::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    return CreatedJson(Json(result));
}

#[delete("/holidays/<id>")]
pub fn destroy(id: i32) -> Custom<String> {
    let conn = &mut db_connection();

    match find_row(id, conn) {
        Ok(Some(_record)) => {
            delete(holidays::dsl::holidays.find(id))
                .execute(conn)
                .expect("Error loading holidays");
            Custom(Status::NoContent, "".to_string())
        },
        Ok(None) => Custom(Status::NotFound, to_resp(format!("Currency {} not found!", id))),
        Err(x) => Custom(Status::InternalServerError, to_resp(format!("Internal error {}",x)))
    }
}

