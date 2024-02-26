use crate::schema::calendars;
use diesel::Queryable;
use diesel::Selectable;
use diesel::Identifiable;
use diesel::Insertable;
use serde::Serialize;
use serde::Deserialize;
use std::fmt::Debug;

#[derive(Debug, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = calendars)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Calendar {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = calendars)]
pub struct NewCalendar<'a> {
    pub name: &'a str,
}
