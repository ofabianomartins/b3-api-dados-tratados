use rocket::get;
use rocket::response::status::NoContent;
use rocket::serde::json::Json;

use diesel::SelectableHelper;
use diesel::RunQueryDsl;
use diesel::query_dsl::QueryDsl;
use diesel::insert_into;
use diesel::delete;

use crate::connections::db_connection;
use crate::models::Segment;
use crate::models::NewSegment;
use crate::schema::segments::dsl::*;

#[get("/segments")]
pub fn index() -> Json<Vec<Segment>> {
    let conn = &mut db_connection();
    let results = segments
        .select(Segment::as_select())
        .load(conn)
        .expect("Error loading segments");
    return Json(results);
}

#[delete("/segments/<segment_id>")]
pub fn destroy(segment_id: i32) -> NoContent {
    let conn = &mut db_connection();
    delete(segments.find(segment_id))
        .execute(conn)
        .expect("Error loading segments");
    return NoContent;
}

#[derive(Responder)]
#[response(status = 201, content_type = "json")]
pub struct CreatedJson(Json<Segment>);

#[post("/segments", format="json", data = "<new_segment>")]
pub async fn create(new_segment: Json<NewSegment<'_>>) -> CreatedJson {
    let conn = &mut db_connection();
    let result = insert_into(segments)
        .values(&*new_segment)
        .returning(Segment::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    return CreatedJson(Json(result));
}

