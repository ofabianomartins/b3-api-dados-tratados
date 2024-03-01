use crate::schema::calendars;
use crate::schema::holidays;
use crate::schema::currencies;
use diesel::Queryable;
use diesel::Selectable;
use diesel::Identifiable;
use diesel::Insertable;
use serde::Serialize;
use serde::Deserialize;
use std::fmt::Debug;
use chrono::NaiveDate;

#[derive(Debug, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = calendars)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Calendar {
    pub id: i32,
    pub name: String,
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = calendars)]
pub struct NewCalendar<'a> {
    pub name: &'a str,
    pub code: &'a str,
}

#[derive(Debug, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = holidays)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Holiday {
    pub id: i32,
    pub name: String,
    pub date: NaiveDate,
    pub calendar_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = holidays)]
pub struct NewHoliday<'a> {
    pub name: &'a str,
    pub date: NaiveDate,
    pub calendar_id: i32,
}

#[derive(Debug, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = currencies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Currency {
    pub id: i32,
    pub name: String,
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = currencies)]
pub struct NewCurrency<'a> {
    pub name: &'a str,
    pub code: &'a str,
}
