use redis::Commands;

mod connections;
mod services;
mod schema;
mod models;
mod utils;

use std::str::FromStr;
use crate::connections::db_connection;

fn main() {
    let conn = &mut connections::redis_connection();
    let conn_db = &mut db_connection();

    let business_calendar = &mut utils::business_calendar::BusinessCalendar::new(
        String::from_str("2024-03-01").expect("Date format problem!"),
        String::from_str("2024-03-10").expect("Date format problem!"),
        Vec::new()
    );

    let mut quote_service = services::quote_service::QuoteService::new(conn_db, business_calendar);

    loop {
        let return_value: Vec<(String, isize)> = conn
            .zrangebyscore_limit_withscores("quote_queue", "-inf", "+inf", 0, 1)
            .expect("Redis ERROR");
        for item in return_value {
            quote_service.process_quote(&item.0);
            let result: i32 = conn.zrem("quote_queue", item.0).expect("ZREM failed!");
        }
    }
}
