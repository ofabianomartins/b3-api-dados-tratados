use diesel::Queryable;
use diesel::Selectable;
use diesel::Identifiable;
use diesel::Insertable;

use serde::Serialize;
use serde::Deserialize;

use std::fmt::Debug;

use crate::schema::*;

#[derive(Debug, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = indicators)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Indicator {
    pub id: i32,
    pub name: String,
    pub symbol: String,
    pub description: String,
    pub indicator_type: String
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = indicators)]
pub struct NewIndicator<'a> {
    pub name: &'a str,
    pub symbol: &'a str,
    pub description: &'a str,
    pub indicator_type: &'a str
}
