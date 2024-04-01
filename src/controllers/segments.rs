use rocket::get;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json;
use rocket::serde::json::Json;

use diesel::PgConnection;
use diesel::SelectableHelper;
use diesel::RunQueryDsl;
use diesel::query_dsl::QueryDsl;
use diesel::QueryResult;
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::insert_into;
use diesel::delete;

use uuid::Uuid;

use crate::connections::db_connection;
use crate::models::segment::Segment;
use crate::models::segment::NewSegment;
use crate::models::segment::ExternalSegment;
use crate::schema::segments;
use crate::controllers::to_resp;

#[get("/segments")]
pub fn index() -> Json<Vec<ExternalSegment>> {
    let conn = &mut db_connection();
    let results = segments::dsl::segments
        .select(ExternalSegment::as_select())
        .load(conn)
        .expect("Error loading segments");
    return Json(results);
}

// Define a function to search for a row by UUID
fn find_row_by_uuid(uuid: Uuid, conn: &mut PgConnection) -> QueryResult<Option<ExternalSegment>> {
    return segments::dsl::segments
        .filter(segments::dsl::uuid.eq(uuid))
        .select(ExternalSegment::as_select())
        .first(conn)
        .optional();
}

#[get("/segments/<segment_uuid>")]
pub fn show(segment_uuid: &str) -> Custom<String> {
    let conn = &mut db_connection();

    match Uuid::parse_str(segment_uuid) {
        Ok(x) => {
            match find_row_by_uuid(x, conn) {
                Ok(Some(row)) => Custom(Status::Ok, json::to_string(&row).unwrap()),
                Ok(None) => Custom(Status::NotFound, to_resp(format!("Currency {} not found!", segment_uuid))),
                Err(x) => Custom(Status::InternalServerError, to_resp(format!("Internal error {}",x)))
            }
        },
        Err(x) => Custom(Status::UnprocessableEntity, to_resp(format!("uuid {} wrong format!", x)))
    }
}


#[derive(Responder)]
#[response(status = 201, content_type = "json")]
pub struct CreatedJson(Json<Segment>);

#[post("/segments", format="json", data = "<new_segment>")]
pub async fn create(new_segment: Json<NewSegment<'_>>) -> CreatedJson {
    let conn = &mut db_connection();
    let result = insert_into(segments::dsl::segments)
        .values(&*new_segment)
        .returning(Segment::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    return CreatedJson(Json(result));
}

#[delete("/segments/<segment_uuid>")]
pub fn destroy(segment_uuid: &str) -> Custom<String> {
    let conn = &mut db_connection();

    match Uuid::parse_str(segment_uuid) {
        Ok(x) => {
            match find_row_by_uuid(x, conn) {
                Ok(Some(_record)) => {
                    delete(segments::dsl::segments)
                        .filter(segments::dsl::uuid.eq(x))
                        .execute(conn)
                        .expect("Error loading segments");
                    Custom(Status::NoContent, "".to_string())
                },
                Ok(None) => Custom(Status::NotFound, to_resp(format!("Segment {} not found!", segment_uuid))),
                Err(x) => Custom(Status::InternalServerError, to_resp(format!("Internal error {}",x)))
            }
        },
        Err(x) => Custom(Status::UnprocessableEntity, to_resp(format!("uuid {} wrong format!", x)))
    }
}

