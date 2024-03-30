use diesel::Queryable;
use diesel::Selectable;
use diesel::Identifiable;
use diesel::Insertable;

use serde::Serialize;
use serde::Deserialize;

use chrono::NaiveDateTime;

use uuid::Uuid;

use std::fmt::Debug;

use crate::schema::*;

#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = calendars)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Calendar {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub uuid: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = calendars)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ExternalCalendar {
    pub name: String,
    pub code: String,
    pub uuid: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = calendars)]
pub struct NewCalendar<'a> {
    pub name: &'a str,
    pub code: &'a str,
}
