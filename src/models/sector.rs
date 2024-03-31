use diesel::Queryable;
use diesel::Selectable;
use diesel::Identifiable;
use diesel::Insertable;

use serde::Serialize;
use serde::Deserialize;

use chrono::NaiveDateTime;

use std::fmt::Debug;

use uuid::Uuid;

use crate::schema::*;

#[derive(Debug, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = sectors)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Sector {
    pub id: i32,
    pub name: String,
    pub uuid: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = sectors)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ExternalSector {
    pub name: String,
    pub uuid: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = sectors)]
pub struct NewSector<'a> {
    pub name: &'a str,
}
