use rocket::get;
use rocket::http::Status;
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

use crate::connections::db_connection;
use crate::models::sector::NewSector;
use crate::models::sector::Sector;
use crate::schema::sectors;

use crate::controllers::to_resp;

#[get("/sectors")]
pub fn index() -> Json<Vec<Sector>> {
    let conn = &mut db_connection();
    let results = sectors::dsl::sectors
        .select(Sector::as_select())
        .load(conn)
        .expect("Error loading sectors");
    return Json(results);
}

// Define a function to search for a row by UUID
fn find_row(id: i32, conn: &mut PgConnection) -> QueryResult<Option<Sector>> {
    return sectors::dsl::sectors
        .find(id)
        .select(Sector::as_select())
        .first(conn)
        .optional();
}

#[get("/sectors/<id>")]
pub fn show(id: i32) -> Custom<String> {
    let conn = &mut db_connection();

    match find_row(id, conn) {
        Ok(Some(row)) => Custom(Status::Ok, json::to_string(&row).unwrap()),
        Ok(None) => Custom(Status::NotFound, to_resp(format!("Sector {} not found!", id))),
        Err(x) => Custom(Status::InternalServerError, to_resp(format!("Internal error {}",x)))
    }
}

#[derive(Responder)]
#[response(status = 201, content_type = "json")]
pub struct CreatedJson(Json<Sector>);

#[post("/sectors", format="json", data = "<new_sector>")]
pub async fn create(new_sector: Json<NewSector<'_>>) -> CreatedJson {
    let conn = &mut db_connection();
    let result = insert_into(sectors::dsl::sectors)
        .values(&*new_sector)
        .returning(Sector::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    return CreatedJson(Json(result));
}

#[put("/sectors/<id>", format="json", data="<sector>")]
pub fn update_action(id: i32, sector: Json<NewSector<'_>>) -> Custom<String> {
    let conn = &mut db_connection();

    match find_row(id, conn) {
        Ok(Some(_record)) => {
            let row = update(sectors::dsl::sectors.find(id))
                .set(sectors::dsl::name.eq(sector.name))
                .returning(Sector::as_returning())
                .execute(conn)
                .expect("Error loading sectors");
            Custom(Status::Ok, json::to_string(&row).unwrap())
        },
        Ok(None) => Custom(Status::NotFound, to_resp(format!("Currency {} not found!", id))),
        Err(x) => Custom(Status::InternalServerError, to_resp(format!("Internal error {}",x)))
    }
}

#[delete("/sectors/<id>")]
pub fn destroy(id: i32) -> Custom<String> {
    let conn = &mut db_connection();

    match find_row(id, conn) {
        Ok(Some(_record)) => {
            delete(sectors::dsl::sectors.find(id))
                .execute(conn)
                .expect("Error loading sectors");
            Custom(Status::NoContent, "".to_string())
        },
        Ok(None) => Custom(Status::NotFound, to_resp(format!("Sector {} not found!", id))),
        Err(x) => Custom(Status::InternalServerError, to_resp(format!("Internal error {}",x)))
    }
}

