use rocket::get;
use rocket::http::Status;
use rocket::response::status::NoContent;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::serde::json;

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
use crate::models::currency::Currency;
use crate::models::currency::NewCurrency;
use crate::models::currency::ExternalCurrency;
use crate::schema::currencies;
use crate::controllers::to_resp;

#[get("/currencies")]
pub fn index() -> Json<Vec<Currency>> {
    let conn = &mut db_connection();
    let results = currencies::dsl::currencies
        .select(Currency::as_select())
        .load(conn)
        .expect("Error loading currencies");
    return Json(results);
}

// Define a function to search for a row by UUID
fn find_row_by_uuid(uuid: Uuid, conn: &mut PgConnection) -> QueryResult<Option<ExternalCurrency>> {
    return currencies::dsl::currencies
        .filter(currencies::dsl::uuid.eq(uuid))
        .select(ExternalCurrency::as_select())
        .first(conn)
        .optional();
}

#[get("/currencies/<currency_uuid>")]
pub fn show(currency_uuid: &str) -> Custom<String> {
    let conn = &mut db_connection();

    match Uuid::parse_str(currency_uuid) {
        Ok(x) => {
            match find_row_by_uuid(x, conn) {
                Ok(Some(row)) => Custom(Status::Ok, json::to_string(&row).unwrap()),
                Ok(None) => Custom(Status::NotFound, to_resp(format!("Currency {} not found!", currency_uuid))),
                Err(x) => Custom(Status::InternalServerError, to_resp(format!("Internal error {}",x)))
            }
        },
        Err(x) => Custom(Status::UnprocessableEntity, to_resp(format!("uuid {} wrong format!", x)))
    }
}


#[derive(Responder)]
#[response(status = 201, content_type = "json")]
pub struct CreatedJson(Json<Currency>);

#[post("/currencies", format="json", data = "<new_currency>")]
pub async fn create(new_currency: Json<NewCurrency<'_>>) -> CreatedJson {
    let conn = &mut db_connection();
    let result = insert_into(currencies::dsl::currencies)
        .values(&*new_currency)
        .returning(Currency::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    return CreatedJson(Json(result));
}

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
pub struct UpdatedJson(Json<Currency>);

#[put("/currencies/<currency_id>", format="json", data="<currency>")]
pub fn update_action(currency_id: i32, currency: Json<NewCurrency<'_>>) -> UpdatedJson {
    let conn = &mut db_connection();
    update(currencies::dsl::currencies.find(currency_id))
        .set((
            currencies::dsl::name.eq(currency.name),
            currencies::dsl::code.eq(currency.code)
        ))
        .returning(Currency::as_returning())
        .execute(conn)
        .expect("Error loading currencies");

    let result = currencies::dsl::currencies
        .find(currency_id)
        .select(Currency::as_select())
        .first(conn)
        .expect("Error loading currencies");
    return UpdatedJson(Json(result));
}

#[delete("/currencies/<currency_id>")]
pub fn destroy(currency_id: i32) -> NoContent {
    let conn = &mut db_connection();
    delete(currencies::dsl::currencies.find(currency_id))
        .execute(conn)
        .expect("Error loading currencies");
    return NoContent;
}

