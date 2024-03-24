use rocket::get;
use rocket::response::status::NoContent;
use rocket::serde::json::Json;

use diesel::SelectableHelper;
use diesel::RunQueryDsl;
use diesel::query_dsl::QueryDsl;
use diesel::insert_into;
use diesel::delete;

use crate::connections::db_connection;
use crate::models::Company;
use crate::models::NewCompany;
use crate::schema::companies::dsl::*;

#[get("/companies")]
pub fn index() -> Json<Vec<Company>> {
    let conn = &mut db_connection();
    let results = companies
        .select(Company::as_select())
        .load(conn)
        .expect("Error loading companies");
    return Json(results);
}

#[delete("/companies/<company_id>")]
pub fn destroy(company_id: i32) -> NoContent {
    let conn = &mut db_connection();
    delete(companies.find(company_id))
        .execute(conn)
        .expect("Error loading companies");
    return NoContent;
}

#[derive(Responder)]
#[response(status = 201, content_type = "json")]
pub struct CreatedJson(Json<Company>);

#[post("/companies", format="json", data = "<new_company>")]
pub async fn create(new_company: Json<NewCompany<'_>>) -> CreatedJson {
    let conn = &mut db_connection();
    let result = insert_into(companies)
        .values(&*new_company)
        .returning(Company::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    return CreatedJson(Json(result));
}

