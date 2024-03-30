use rocket::get;
use rocket::response::status::NoContent;
use rocket::serde::json::Json;

use diesel::SelectableHelper;
use diesel::RunQueryDsl;
use diesel::query_dsl::QueryDsl;
use diesel::insert_into;
use diesel::delete;

use crate::connections::db_connection;
use crate::models::theory_portfolio::TheoryPortfolio;
use crate::models::theory_portfolio::NewTheoryPortfolio;
use crate::schema::theory_portfolios::dsl::*;

#[get("/theory_portfolios")]
pub fn index() -> Json<Vec<TheoryPortfolio>> {
    let conn = &mut db_connection();
    let results = theory_portfolios
        .select(TheoryPortfolio::as_select())
        .load(conn)
        .expect("Error loading theory_portfolios");
    return Json(results);
}

#[derive(Responder)]
#[response(status = 201, content_type = "json")]
pub struct CreatedJson(Json<TheoryPortfolio>);

#[post("/theory_portfolios", format="json", data = "<new_theory_portfolio>")]
pub async fn create(new_theory_portfolio: Json<NewTheoryPortfolio>) -> CreatedJson {
    let conn = &mut db_connection();
    let result = insert_into(theory_portfolios)
        .values(&*new_theory_portfolio)
        .returning(TheoryPortfolio::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    return CreatedJson(Json(result));
}

#[delete("/theory_portfolios/<theory_portfolio_id>")]
pub fn destroy(theory_portfolio_id: i32) -> NoContent {
    let conn = &mut db_connection();
    delete(theory_portfolios.find(theory_portfolio_id))
        .execute(conn)
        .expect("Error loading theory_portfolios");
    return NoContent;
}
