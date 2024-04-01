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
use diesel::ExpressionMethods;
use diesel::query_dsl::QueryDsl;
use diesel::insert_into;
use diesel::delete;

use uuid::Uuid;

use crate::connections::db_connection;
use crate::models::indicator::NewIndicator;
use crate::models::indicator::ExternalIndicator;
use crate::schema::indicators;
use crate::controllers::to_resp;

#[get("/indicators")]
pub fn index() -> Json<Vec<ExternalIndicator>> {
    let conn = &mut db_connection();
    let results = indicators::dsl::indicators
        .select(ExternalIndicator::as_select())
        .load(conn)
        .expect("Error loading indicators");
    return Json(results);
}

// Define a function to search for a row by UUID
fn find_row_by_uuid(uuid: Uuid, conn: &mut PgConnection) -> QueryResult<Option<ExternalIndicator>> {
    return indicators::dsl::indicators
        .filter(indicators::dsl::uuid.eq(uuid))
        .select(ExternalIndicator::as_select())
        .first(conn)
        .optional();
}

#[get("/indicators/<indicator_uuid>")]
pub fn show(indicator_uuid: &str) -> Custom<String> {
    let conn = &mut db_connection();

    match Uuid::parse_str(indicator_uuid) {
        Ok(x) => {
            match find_row_by_uuid(x, conn) {
                Ok(Some(row)) => Custom(Status::Ok, json::to_string(&row).unwrap()),
                Ok(None) => Custom(Status::NotFound, to_resp(format!("Indicator {} not found!", indicator_uuid))),
                Err(x) => Custom(Status::InternalServerError, to_resp(format!("Internal error {}",x)))
            }
        },
        Err(x) => Custom(Status::UnprocessableEntity, to_resp(format!("uuid {} wrong format!", x)))
    }
}

#[derive(Responder)]
#[response(status = 201, content_type = "json")]
pub struct CreatedJson(Json<ExternalIndicator>);

#[post("/indicators", format="json", data = "<new_indicator>")]
pub async fn create(new_indicator: Json<NewIndicator<'_>>) -> CreatedJson {
    let conn = &mut db_connection();
    let result = insert_into(indicators::dsl::indicators)
        .values(&*new_indicator)
        .returning(ExternalIndicator::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    return CreatedJson(Json(result));
}

#[delete("/indicators/<indicator_uuid>")]
pub fn destroy(indicator_uuid: &str) -> Custom<String> {
    let conn = &mut db_connection();

    match Uuid::parse_str(indicator_uuid) {
        Ok(x) => {
            match find_row_by_uuid(x, conn) {
                Ok(Some(_record)) => {
                    delete(indicators::dsl::indicators)
                        .filter(indicators::dsl::uuid.eq(x))
                        .execute(conn)
                        .expect("Error loading indicators");
                    Custom(Status::NoContent, "".to_string())
                },
                Ok(None) => Custom(Status::NotFound, to_resp(format!("Currency {} not found!", indicator_uuid))),
                Err(x) => Custom(Status::InternalServerError, to_resp(format!("Internal error {}",x)))
            }
        },
        Err(x) => Custom(Status::UnprocessableEntity, to_resp(format!("uuid {} wrong format!", x)))
    }
}

