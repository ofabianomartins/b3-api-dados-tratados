use rocket::get;
use rocket::response::status::NoContent;
use rocket::serde::json::Json;

use diesel::SelectableHelper;
use diesel::RunQueryDsl;
use diesel::query_dsl::QueryDsl;
use diesel::insert_into;
use diesel::delete;

use crate::connections::db_connection;
use crate::models::subsector::Subsector;
use crate::models::subsector::NewSubsector;
use crate::schema::subsectors::dsl::*;

#[get("/subsectors")]
pub fn index() -> Json<Vec<Subsector>> {
    let conn = &mut db_connection();
    let results = subsectors
        .select(Subsector::as_select())
        .load(conn)
        .expect("Error loading subsectors");
    return Json(results);
}

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
pub struct ShowJson(Json<Subsector>);

#[get("/subsectors/<subsector_id>")]
pub fn show(subsector_id: i32) -> ShowJson {
    let conn = &mut db_connection();
    let result = subsectors
        .find(subsector_id)
        .select(Subsector::as_select())
        .first(conn)
        .expect("Error loading segments");
    return ShowJson(Json(result));
}


#[derive(Responder)]
#[response(status = 201, content_type = "json")]
pub struct CreatedJson(Json<Subsector>);

#[post("/subsectors", format="json", data = "<new_subsector>")]
pub async fn create(new_subsector: Json<NewSubsector<'_>>) -> CreatedJson {
    let conn = &mut db_connection();
    let result = insert_into(subsectors)
        .values(&*new_subsector)
        .returning(Subsector::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    return CreatedJson(Json(result));
}

#[delete("/subsectors/<subsector_id>")]
pub fn destroy(subsector_id: i32) -> NoContent {
    let conn = &mut db_connection();
    delete(subsectors.find(subsector_id))
        .execute(conn)
        .expect("Error loading subsectors");
    return NoContent;
}

