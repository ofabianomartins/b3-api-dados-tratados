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
#[diesel(table_name = segments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Segment {
    pub id: i32,
    pub name: String,
    pub subsector_id: i32,
    pub uuid: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = segments)]
pub struct NewSegment<'a> {
    pub name: &'a str,
    pub subsector_id: i32
}

