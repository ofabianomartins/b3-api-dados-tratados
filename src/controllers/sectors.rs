use rocket::get;
use rocket::response::status::NoContent;
use rocket::serde::json::Json;

use diesel::SelectableHelper;
use diesel::RunQueryDsl;
use diesel::query_dsl::QueryDsl;
use diesel::insert_into;
use diesel::delete;

use crate::connections::db_connection;
use crate::models::Sector;
use crate::models::NewSector;
use crate::schema::sectors::dsl::*;

#[get("/sectors")]
pub fn index() -> Json<Vec<Sector>> {
    let conn = &mut db_connection();
    let results = sectors
        .select(Sector::as_select())
        .load(conn)
        .expect("Error loading sectors");
    return Json(results);
}

#[delete("/sectors/<sector_id>")]
pub fn destroy(sector_id: i32) -> NoContent {
    let conn = &mut db_connection();
    delete(sectors.find(sector_id))
        .execute(conn)
        .expect("Error loading sectors");
    return NoContent;
}

#[derive(Responder)]
#[response(status = 201, content_type = "json")]
pub struct CreatedJson(Json<Sector>);

#[post("/sectors", format="json", data = "<new_sector>")]
pub async fn create(new_sector: Json<NewSector<'_>>) -> CreatedJson {
    let conn = &mut db_connection();
    let result = insert_into(sectors)
        .values(&*new_sector)
        .returning(Sector::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    return CreatedJson(Json(result));
}

