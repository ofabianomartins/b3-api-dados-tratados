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
use crate::models::indicator::NewIndicator;
use crate::models::indicator::Indicator;
use crate::schema::indicators;
use crate::controllers::to_resp;

#[get("/indicators")]
pub fn index() -> Json<Vec<Indicator>> {
    let conn = &mut db_connection();
    let results = indicators::dsl::indicators
        .select(Indicator::as_select())
        .load(conn)
        .expect("Error loading indicators");
    return Json(results);
}

// Define a function to search for a row by UUID
fn find_row(id: i32, conn: &mut PgConnection) -> QueryResult<Option<Indicator>> {
    return indicators::dsl::indicators
        .find(id)
        .select(Indicator::as_select())
        .first(conn)
        .optional();
}

#[get("/indicators/<id>")]
pub fn show(id: i32) -> Custom<String> {
    let conn = &mut db_connection();

    match find_row(id, conn) {
        Ok(Some(row)) => Custom(Status::Ok, json::to_string(&row).unwrap()),
        Ok(None) => Custom(Status::NotFound, to_resp(format!("Indicator {} not found!", id))),
        Err(x) => Custom(Status::InternalServerError, to_resp(format!("Internal error {}",x)))
    }
}

#[derive(Responder)]
#[response(status = 201, content_type = "json")]
pub struct CreatedJson(Json<Indicator>);

#[post("/indicators", format="json", data = "<new_indicator>")]
pub async fn create(new_indicator: Json<NewIndicator<'_>>) -> CreatedJson {
    let conn = &mut db_connection();
    let result = insert_into(indicators::dsl::indicators)
        .values(&*new_indicator)
        .returning(Indicator::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    return CreatedJson(Json(result));
}

#[delete("/indicators/<id>")]
pub fn destroy(id: i32) -> Custom<String> {
    let conn = &mut db_connection();

    match find_row(id, conn) {
        Ok(Some(_record)) => {
            delete(indicators::dsl::indicators.find(id))
                .execute(conn)
                .expect("Error loading indicators");
            Custom(Status::NoContent, "".to_string())
        },
        Ok(None) => Custom(Status::NotFound, to_resp(format!("Currency {} not found!", id))),
        Err(x) => Custom(Status::InternalServerError, to_resp(format!("Internal error {}",x)))
    }
}

