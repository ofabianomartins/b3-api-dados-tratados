use diesel::Queryable;
use diesel::Selectable;
use diesel::Identifiable;
use diesel::Insertable;

use serde::Serialize;
use serde::Deserialize;

use std::fmt::Debug;

use crate::schema::*;

#[derive(Debug, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = subsectors)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Subsector {
    pub id: i32,
    pub name: String,
    pub sector_id: i32
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = subsectors)]
pub struct NewSubsector<'a> {
    pub name: &'a str,
    pub sector_id: i32
}
