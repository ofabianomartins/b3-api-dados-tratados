use rocket::get;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json;
use rocket::serde::json::Json;

use diesel::PgConnection;
use diesel::SelectableHelper;
use diesel::RunQueryDsl;
use diesel::OptionalExtension;
use diesel::QueryResult;
use diesel::query_dsl::QueryDsl;
use diesel::delete;

use redis::Commands;

use chrono::Utc;

use crate::connections::db_connection;
use crate::connections::redis_connection;
use crate::models::quote::Quote;
use crate::schema::quotes;

use crate::controllers::to_resp;

#[get("/quotes")]
pub fn index() -> Json<Vec<Quote>> {
    let conn = &mut db_connection();
    let results = quotes::dsl::quotes
        .select(Quote::as_select())
        .load(conn)
        .expect("Error loading tickers");
    return Json(results);
}

// Define a function to search for a row by UUID
fn find_row(id: i32, conn: &mut PgConnection) -> QueryResult<Option<Quote>> {
    return quotes::dsl::quotes
        .find(id)
        .select(Quote::as_select())
        .first(conn)
        .optional();
}

#[get("/quotes/<id>")]
pub fn show(id: i32) -> Custom<String> {
    let conn = &mut db_connection();

    match find_row(id, conn) {
        Ok(Some(row)) => Custom(Status::Ok, json::to_string(&row).unwrap()),
        Ok(None) => Custom(Status::NotFound, to_resp(format!("Currency {} not found!", id))),
        Err(x) => Custom(Status::InternalServerError, to_resp(format!("Internal error {}",x)))
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

#[delete("/quotes/<id>")]
pub fn destroy(id: i32) -> Custom<String> {
    let conn = &mut db_connection();

    match find_row(id, conn) {
        Ok(Some(_record)) => {
            delete(quotes::dsl::quotes.find(id))
                .execute(conn)
                .expect("Error loading quotes");
            Custom(Status::NoContent, "".to_string())
        },
        Ok(None) => Custom(Status::NotFound, to_resp(format!("Currency {} not found!", id))),
        Err(x) => Custom(Status::InternalServerError, to_resp(format!("Internal error {}",x)))
    }
}

