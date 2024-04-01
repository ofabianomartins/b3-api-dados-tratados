use rocket::get;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json;
use rocket::serde::json::Json;

use diesel::SelectableHelper;
use diesel::RunQueryDsl;
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::query_dsl::QueryDsl;
use diesel::PgConnection;
use diesel::QueryResult;
use diesel::insert_into;
use diesel::update;
use diesel::delete;

use uuid::Uuid;

use crate::connections::db_connection;
use crate::models::company::Company;
use crate::models::company::NewCompany;
use crate::models::company::ExternalCompany;
use crate::schema::companies;

use crate::controllers::to_resp;

#[get("/companies")]
pub fn index() -> Json<Vec<ExternalCompany>> {
    let conn = &mut db_connection();
    let results = companies::dsl::companies
        .select(ExternalCompany::as_select())
        .load(conn)
        .expect("Error loading companies");
    return Json(results);
}

// Define a function to search for a row by UUID
fn find_row_by_uuid(uuid: Uuid, conn: &mut PgConnection) -> QueryResult<Option<ExternalCompany>> {
    return companies::dsl::companies
        .filter(companies::dsl::uuid.eq(uuid))
        .select(ExternalCompany::as_select())
        .first(conn)
        .optional();
}

#[get("/companies/<company_uuid>")]
pub fn show(company_uuid: &str) -> Custom<String> {
    let conn = &mut db_connection();

    match Uuid::parse_str(company_uuid) {
        Ok(x) => {
            match find_row_by_uuid(x, conn) {
                Ok(Some(row)) => Custom(Status::Ok, json::to_string(&row).unwrap()),
                Ok(None) => Custom(Status::NotFound, to_resp(format!("Company {} not found!", company_uuid))),
                Err(x) => Custom(Status::InternalServerError, to_resp(format!("Internal error {}",x)))
            }
        },
        Err(x) => Custom(Status::UnprocessableEntity, to_resp(format!("uuid {} wrong format!", x)))
    }
}

#[derive(Responder)]
#[response(status = 201, content_type = "json")]
pub struct CreatedJson(Json<Company>);

#[post("/companies", format="json", data = "<new_company>")]
pub async fn create(new_company: Json<NewCompany<'_>>) -> CreatedJson {
    let conn = &mut db_connection();
    let result = insert_into(companies::dsl::companies)
        .values(&*new_company)
        .returning(Company::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    return CreatedJson(Json(result));
}

#[put("/companies/<company_uuid>", format="json", data="<company>")]
pub fn update_action(company_uuid: &str, company: Json<NewCompany<'_>>) -> Custom<String> {
    let conn = &mut db_connection();

    match Uuid::parse_str(company_uuid) {
        Ok(x) => {
            match find_row_by_uuid(x, conn) {
                Ok(Some(_record)) => {
                    let row = update(companies::dsl::companies)
                        .filter(companies::dsl::uuid.eq(x))
                        .set((
                            companies::dsl::name.eq(company.name),
                            companies::dsl::company_type.eq(company.company_type),
                            companies::dsl::cnpj.eq(company.cnpj)
                        ))
                        .returning(ExternalCompany::as_returning())
                        .execute(conn)
                        .expect("Error loading currencies");
                    Custom(Status::Ok, json::to_string(&row).unwrap())
                },
                Ok(None) => Custom(Status::NotFound, to_resp(format!("Currency {} not found!", company_uuid))),
                Err(x) => Custom(Status::InternalServerError, to_resp(format!("Internal error {}",x)))
            }
        },
        Err(x) => Custom(Status::UnprocessableEntity, to_resp(format!("uuid {} wrong format!", x)))
    }
}

#[delete("/companies/<company_uuid>")]
pub fn destroy(company_uuid: &str) -> Custom<String> {
    let conn = &mut db_connection();

    match Uuid::parse_str(company_uuid) {
        Ok(x) => {
            match find_row_by_uuid(x, conn) {
                Ok(Some(_record)) => {
                    delete(companies::dsl::companies)
                        .filter(companies::dsl::uuid.eq(x))
                        .execute(conn)
                        .expect("Error loading companies");
                    Custom(Status::NoContent, "".to_string())
                },
                Ok(None) => Custom(Status::NotFound, to_resp(format!("Currency {} not found!", company_uuid))),
                Err(x) => Custom(Status::InternalServerError, to_resp(format!("Internal error {}",x)))
            }
        },
        Err(x) => Custom(Status::UnprocessableEntity, to_resp(format!("uuid {} wrong format!", x)))
    }
}

