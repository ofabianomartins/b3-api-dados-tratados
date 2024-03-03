use redis::Commands;

mod connections;
mod services;
mod schema;
mod models;
mod utils;

fn main() {
    let conn = &mut connections::redis_connection();

    loop {
        let return_value: Vec<(String, isize)> = conn
            .zrangebyscore_limit_withscores("quote_queue", "-inf", "+inf", 0, 1)
            .expect("Redis ERROR");
        for item in return_value {
            services::quote_service::process_quote(&item.0);
            let _result: i32 = conn.zrem("quote_queue", item.0).expect("ZREM failed!");
        }

    }
}
