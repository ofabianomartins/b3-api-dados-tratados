use rocket::get;
use rocket::response::status::NoContent;
use rocket::serde::json::Json;

use diesel::SelectableHelper;
use diesel::RunQueryDsl;
use diesel::query_dsl::QueryDsl;
use diesel::insert_into;
use diesel::delete;

use crate::connections::establish_connection;
use crate::models::TheoryPortfolioTransaction;
use crate::models::NewTheoryPortfolioTransaction;
use crate::schema::theory_portfolio_transactions::dsl::*;

#[get("/theory_portfolio_transactions")]
pub fn index() -> Json<Vec<TheoryPortfolioTransaction>> {
    let conn = &mut establish_connection();
    let results = theory_portfolio_transactions
        .select(TheoryPortfolioTransaction::as_select())
        .load(conn)
        .expect("Error loading theory_portfolio_transactions");
    return Json(results);
}

#[delete("/theory_portfolio_transactions/<theory_portfolio_transaction_id>")]
pub fn destroy(theory_portfolio_transaction_id: i32) -> NoContent {
    let conn = &mut establish_connection();
    delete(theory_portfolio_transactions.find(theory_portfolio_transaction_id))
        .execute(conn)
        .expect("Error loading theory_portfolio_transactions");
    return NoContent;
}

#[derive(Responder)]
#[response(status = 201, content_type = "json")]
pub struct CreatedJson(Json<TheoryPortfolioTransaction>);

#[post("/theory_portfolio_transactions", format="json", data = "<new_theory_portfolio_transaction>")]
pub async fn create(new_theory_portfolio_transaction: Json<NewTheoryPortfolioTransaction>) -> CreatedJson {
    let conn = &mut establish_connection();
    let result = insert_into(theory_portfolio_transactions)
        .values(&*new_theory_portfolio_transaction)
        .returning(TheoryPortfolioTransaction::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    return CreatedJson(Json(result));
}

