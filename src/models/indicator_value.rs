use diesel::Queryable;
use diesel::Selectable;
use diesel::Identifiable;
use diesel::Insertable;

use serde::Serialize;
use serde::Deserialize;

use std::fmt::Debug;

use chrono::NaiveDate;
use bigdecimal::BigDecimal;

use crate::schema::*;

#[derive(Debug, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = indicator_values)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct IndicatorValue {
    pub id: i32,
    pub date: NaiveDate,
    pub indicator_id: i32,
    pub company_id: i32,
    pub close: BigDecimal
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = indicator_values)]
pub struct NewIndicatorValue {
    pub date: NaiveDate,
    pub company_id: i32,
    pub indicator_id: i32,
    pub close: BigDecimal
}
