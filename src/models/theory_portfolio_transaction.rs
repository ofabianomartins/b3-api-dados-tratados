use diesel::Queryable;
use diesel::Selectable;
use diesel::Identifiable;
use diesel::Insertable;

use serde::Serialize;
use serde::Deserialize;

use std::fmt::Debug;

use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use chrono::NaiveDateTime;

use uuid::Uuid;

use crate::schema::*;

#[derive(Debug, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = theory_portfolio_transactions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TheoryPortfolioTransaction {
    pub id: i32,
    pub date: NaiveDate,
    pub quantity: BigDecimal,
    pub ticker_id: i32,
    pub theory_portfolio_id: i32,
    pub uuid: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = theory_portfolio_transactions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ExternalTheoryPortfolioTransaction {
    pub date: NaiveDate,
    pub quantity: BigDecimal,
    pub ticker_id: i32,
    pub theory_portfolio_id: i32,
    pub uuid: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = theory_portfolio_transactions)]
pub struct NewTheoryPortfolioTransaction {
    pub date: NaiveDate,
    pub quantity: BigDecimal,
    pub ticker_id: i32,
    pub theory_portfolio_id: i32,
}
