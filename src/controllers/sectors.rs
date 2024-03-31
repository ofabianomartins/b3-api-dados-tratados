use rocket::get;
use rocket::http::Status;
use rocket::response::status::NoContent;
use rocket::response::status::Custom;
use rocket::serde::json;
use rocket::serde::json::Json;

use diesel::PgConnection;
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
use crate::models::sector::Sector;
use crate::models::sector::NewSector;
use crate::models::sector::ExternalSector;
use crate::schema::sectors;

use crate::controllers::to_resp;

#[get("/sectors")]
pub fn index() -> Json<Vec<ExternalSector>> {
    let conn = &mut db_connection();
    let results = sectors::dsl::sectors
        .select(ExternalSector::as_select())
        .load(conn)
        .expect("Error loading sectors");
    return Json(results);
}

// Define a function to search for a row by UUID
fn find_row_by_uuid(uuid: Uuid, conn: &mut PgConnection) -> QueryResult<Option<ExternalSector>> {
    return sectors::dsl::sectors
        .filter(sectors::dsl::uuid.eq(uuid))
        .select(ExternalSector::as_select())
        .first(conn)
        .optional();
}

#[get("/sectors/<sector_uuid>")]
pub fn show(sector_uuid: &str) -> Custom<String> {
    let conn = &mut db_connection();

    match Uuid::parse_str(sector_uuid) {
        Ok(x) => {
            match find_row_by_uuid(x, conn) {
                Ok(Some(row)) => Custom(Status::Ok, json::to_string(&row).unwrap()),
                Ok(None) => Custom(Status::NotFound, to_resp(format!("Sector {} not found!", sector_uuid))),
                Err(x) => Custom(Status::InternalServerError, to_resp(format!("Internal error {}",x)))
            }
        },
        Err(x) => Custom(Status::UnprocessableEntity, to_resp(format!("uuid {} wrong format!", x)))
    }
}

#[derive(Responder)]
#[response(status = 201, content_type = "json")]
pub struct CreatedJson(Json<ExternalSector>);

#[post("/sectors", format="json", data = "<new_sector>")]
pub async fn create(new_sector: Json<NewSector<'_>>) -> CreatedJson {
    let conn = &mut db_connection();
    let result = insert_into(sectors::dsl::sectors)
        .values(&*new_sector)
        .returning(ExternalSector::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    return CreatedJson(Json(result));
}

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
pub struct UpdatedJson(Json<ExternalSector>);

#[put("/sectors/<sector_id>", format="json", data="<sector>")]
pub fn update_action(sector_id: i32, sector: Json<NewSector<'_>>) -> UpdatedJson {
    let conn = &mut db_connection();
    update(sectors::dsl::sectors.find(sector_id))
        .set(sectors::dsl::name.eq(sector.name))
        .returning(Sector::as_returning())
        .execute(conn)
        .expect("Error loading sectors");

    let result = sectors::dsl::sectors
        .find(sector_id)
        .select(ExternalSector::as_select())
        .first(conn)
        .expect("Error loading sectors");
    return UpdatedJson(Json(result));
}

#[delete("/sectors/<sector_id>")]
pub fn destroy(sector_id: i32) -> NoContent {
    let conn = &mut db_connection();
    delete(sectors::dsl::sectors.find(sector_id))
        .execute(conn)
        .expect("Error loading sectors");
    return NoContent;
}

