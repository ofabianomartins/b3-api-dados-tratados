use rocket::get;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json;
use rocket::serde::json::Json;

use diesel::PgConnection;
use diesel::SelectableHelper;
use diesel::RunQueryDsl;
use diesel::OptionalExtension;
use diesel::ExpressionMethods;
use diesel::QueryResult;
use diesel::query_dsl::QueryDsl;
use diesel::delete;

use redis::Commands;

use chrono::Utc;

use uuid::Uuid;

use crate::connections::db_connection;
use crate::connections::redis_connection;
use crate::models::quote::Quote;
use crate::models::quote::ExternalQuote;
use crate::schema::quotes;

use crate::controllers::to_resp;

#[get("/quotes")]
pub fn index() -> Json<Vec<ExternalQuote>> {
    let conn = &mut db_connection();
    let results = quotes::dsl::quotes
        .select(ExternalQuote::as_select())
        .load(conn)
        .expect("Error loading tickers");
    return Json(results);
}

// Define a function to search for a row by UUID
fn find_row_by_uuid(uuid: Uuid, conn: &mut PgConnection) -> QueryResult<Option<ExternalQuote>> {
    return quotes::dsl::quotes
        .filter(quotes::dsl::uuid.eq(uuid))
        .select(ExternalQuote::as_select())
        .first(conn)
        .optional();
}

#[get("/quotes/<quote_uuid>")]
pub fn show(quote_uuid: &str) -> Custom<String> {
    let conn = &mut db_connection();

    match Uuid::parse_str(quote_uuid) {
        Ok(x) => {
            match find_row_by_uuid(x, conn) {
                Ok(Some(row)) => Custom(Status::Ok, json::to_string(&row).unwrap()),
                Ok(None) => Custom(Status::NotFound, to_resp(format!("Currency {} not found!", quote_uuid))),
                Err(x) => Custom(Status::InternalServerError, to_resp(format!("Internal error {}",x)))
            }
        },
        Err(x) => Custom(Status::UnprocessableEntity, to_resp(format!("uuid {} wrong format!", x)))
    }
}

#[derive(Responder)]
#[response(status = 201, content_type = "json")]
pub struct CreatedJson(Json<Quote>);

#[post("/quotes", format="json", data = "<quote_params>")]
pub async fn create(quote_params: &str) -> &'static str {
    let conn = &mut redis_connection();

    let _result: i32 = conn.zadd(
        "quote_queue", 
        quote_params.replace("\n", "").replace(" ", ""),
        Utc::now().timestamp()
    ).expect("ZADD failed!");

    return "Send to be process!"
}

#[delete("/quotes/<quote_uuid>")]
pub fn destroy(quote_uuid: &str) -> Custom<String> {
    let conn = &mut db_connection();

    match Uuid::parse_str(quote_uuid) {
        Ok(x) => {
            match find_row_by_uuid(x, conn) {
                Ok(Some(_record)) => {
                    delete(quotes::dsl::quotes)
                        .filter(quotes::dsl::uuid.eq(x))
                        .execute(conn)
                        .expect("Error loading quotes");
                    Custom(Status::NoContent, "".to_string())
                },
                Ok(None) => Custom(Status::NotFound, to_resp(format!("Currency {} not found!", quote_uuid))),
                Err(x) => Custom(Status::InternalServerError, to_resp(format!("Internal error {}",x)))
            }
        },
        Err(x) => Custom(Status::UnprocessableEntity, to_resp(format!("uuid {} wrong format!", x)))
    }
}

