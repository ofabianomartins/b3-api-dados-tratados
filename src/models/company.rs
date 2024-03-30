use diesel::Queryable;
use diesel::Selectable;
use diesel::Identifiable;
use diesel::Insertable;

use serde::Serialize;
use serde::Deserialize;

use std::fmt::Debug;

use crate::schema::*;

#[derive(Debug, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = companies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Company {
    pub id: i32,
    pub name: String,
    pub company_type: String,
    pub cnpj: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = companies)]
pub struct NewCompany<'a> {
    pub name: &'a str,
    pub company_type: &'a str,
    pub cnpj: &'a str,
}
