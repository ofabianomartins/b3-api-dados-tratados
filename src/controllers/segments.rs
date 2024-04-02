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
use diesel::OptionalExtension;
use diesel::insert_into;
use diesel::delete;

use crate::connections::db_connection;
use crate::models::segment::Segment;
use crate::models::segment::NewSegment;
use crate::schema::segments;
use crate::controllers::to_resp;

#[get("/segments")]
pub fn index() -> Json<Vec<Segment>> {
    let conn = &mut db_connection();
    let results = segments::dsl::segments
        .select(Segment::as_select())
        .load(conn)
        .expect("Error loading segments");
    return Json(results);
}

// Define a function to search for a row by UUID
fn find_row(id: i32, conn: &mut PgConnection) -> QueryResult<Option<Segment>> {
    return segments::dsl::segments
        .find(id)
        .select(Segment::as_select())
        .first(conn)
        .optional();
}

#[get("/segments/<id>")]
pub fn show(id: i32) -> Custom<String> {
    let conn = &mut db_connection();

    match find_row(id, conn) {
        Ok(Some(row)) => Custom(Status::Ok, json::to_string(&row).unwrap()),
        Ok(None) => Custom(Status::NotFound, to_resp(format!("Currency {} not found!", id))),
        Err(x) => Custom(Status::InternalServerError, to_resp(format!("Internal error {}",x)))
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

#[delete("/segments/<id>")]
pub fn destroy(id: i32) -> Custom<String> {
    let conn = &mut db_connection();

    match find_row(id, conn) {
        Ok(Some(_record)) => {
            delete(segments::dsl::segments.find(id))
                .execute(conn)
                .expect("Error loading segments");
            Custom(Status::NoContent, "".to_string())
        },
        Ok(None) => Custom(Status::NotFound, to_resp(format!("Segment {} not found!", id))),
        Err(x) => Custom(Status::InternalServerError, to_resp(format!("Internal error {}",x)))
    }
}

