use chrono::NaiveDate;

use std::str::FromStr;

use crate::utils::business_calendar::BusinessCalendar;


#[test]
fn test_create_date_calendars() {
    let business_calendar = &mut BusinessCalendar::new(
        String::from_str("2024-03-01").expect("Date format problem!"),
        String::from_str("2024-03-10").expect("Date format problem!"),
        Vec::new()
    );

    assert_eq!(
        business_calendar.adjust(
            NaiveDate::parse_from_str("2024-03-03", "%Y-%m-%d").unwrap()
        ),
        "2024-03-04"
    )
}

#[test]
fn test_create_date_calendars_with_holidays() {
    let mut holidays: Vec<String> = Vec::new();

    holidays.push("2024-03-04".to_string());

    let business_calendar = &mut BusinessCalendar::new(
        String::from_str("2024-03-01").expect("Date format problem!"),
        String::from_str("2024-03-10").expect("Date format problem!"),
        holidays
    );

    assert_eq!(
        business_calendar.adjust(
            NaiveDate::parse_from_str("2024-03-03", "%Y-%m-%d").unwrap()
        ),
        "2024-03-05"
    )
}

#[test]
fn test_adjust_one_year_ago() {
    let holidays: Vec<String> = Vec::new();

    let business_calendar = &mut BusinessCalendar::new(
        String::from_str("2024-03-01").expect("Date format problem!"),
        String::from_str("2024-03-10").expect("Date format problem!"),
        holidays
    );

    assert_eq!(
        business_calendar.advance(
            NaiveDate::parse_from_str("2023-03-02", "%Y-%m-%d").unwrap(),
            0
        ),
        "2023-03-02"
    );
    assert_eq!(business_calendar.start_date, "2023-03-01");
    assert_eq!(business_calendar.end_date, "2024-03-10");
}

#[test]
fn test_adjust_one_year_to_the_future() {
    let holidays: Vec<String> = Vec::new();

    let business_calendar = &mut BusinessCalendar::new(
        String::from_str("2024-03-01").expect("Date format problem!"),
        String::from_str("2024-03-10").expect("Date format problem!"),
        holidays
    );

    assert_eq!(
        business_calendar.advance(
            NaiveDate::parse_from_str("2025-03-02", "%Y-%m-%d").unwrap(),
            0
        ),
        "2025-03-03"
    );
    assert_eq!(business_calendar.start_date, "2024-03-01");
    assert_eq!(business_calendar.end_date, "2025-03-01");
}

