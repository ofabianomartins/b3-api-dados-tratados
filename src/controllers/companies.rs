use rocket::get;
use rocket::response::status::NoContent;
use rocket::serde::json::Json;

use diesel::SelectableHelper;
use diesel::RunQueryDsl;
use diesel::ExpressionMethods;
use diesel::query_dsl::QueryDsl;
use diesel::insert_into;
use diesel::update;
use diesel::delete;

use crate::connections::db_connection;
use crate::models::company::Company;
use crate::models::company::NewCompany;
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

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
pub struct ShowJson(Json<Company>);

#[get("/companies/<company_id>")]
pub fn show(company_id: i32) -> ShowJson {
    let conn = &mut db_connection();
    let result = companies
        .find(company_id)
        .select(Company::as_select())
        .first(conn)
        .expect("Error loading companies");
    return ShowJson(Json(result));
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

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
pub struct UpdatedJson(Json<Company>);

#[put("/companies/<company_id>", format="json", data="<company>")]
pub fn update_action(company_id: i32, company: Json<NewCompany<'_>>) -> UpdatedJson {
    let conn = &mut db_connection();
    update(companies.find(company_id))
        .set((
            name.eq(company.name),
            company_type.eq(company.company_type),
            cnpj.eq(company.cnpj)
        ))
        .returning(Company::as_returning())
        .execute(conn)
        .expect("Error loading companies");

    let result = companies
        .find(company_id)
        .select(Company::as_select())
        .first(conn)
        .expect("Error loading companies");
    return UpdatedJson(Json(result));
}

#[delete("/companies/<company_id>")]
pub fn destroy(company_id: i32) -> NoContent {
    let conn = &mut db_connection();
    delete(companies.find(company_id))
        .execute(conn)
        .expect("Error loading companies");
    return NoContent;
}

