use chrono::NaiveDate;
use chrono::Datelike;
use chrono::Weekday;
use chrono::Duration;

use std::collections::HashMap;

pub struct BusinessCalendar {
    pub start_date: String,
    pub end_date: String,
    pub holidays: Vec<String>,
    pub business_dates: Vec<String>,
    pub business_dates_index: HashMap<String, usize>,
    pub next_business_date_index: HashMap<String, usize>,
    pub prev_business_date_index: HashMap<String, usize>
}

impl BusinessCalendar {

    pub fn new(start_date_str: String, end_date_str: String, holidays: Vec<String>) -> Self {
        let mut object = Self {
            start_date: start_date_str.clone(),
            end_date: end_date_str.clone(),
            holidays: holidays.clone(),
            business_dates: Vec::new(),
            business_dates_index: HashMap::new(),
            next_business_date_index: HashMap::new(),
            prev_business_date_index: HashMap::new()
        };

        object.build(start_date_str, end_date_str, holidays);

        return object;
    }

    pub fn build(&mut self, start_date_str: String, end_date_str: String, holidays: Vec<String>) {
        let mut start_date = NaiveDate::parse_from_str(&start_date_str, "%Y-%m-%d").unwrap();
        let mut end_date = NaiveDate::parse_from_str(&end_date_str, "%Y-%m-%d").unwrap();

        while BusinessCalendar::is_holiday(start_date, &holidays) {
            start_date = start_date.pred_opt().unwrap();
        }
        while BusinessCalendar::is_holiday(end_date, &holidays) {
            end_date = end_date.succ_opt().unwrap();
        }

        self.business_dates.clear();
        self.business_dates_index.clear();

        self.next_business_date_index.clear();
        self.prev_business_date_index.clear();

        let mut i: usize = 0;
        let mut d = start_date.clone();

        while (end_date - d).num_days() >= 0 {
            if BusinessCalendar::is_holiday(d, &holidays) && i > 0{
                self.next_business_date_index.insert(d.format("%Y-%m-%d").to_string(), i);
                self.prev_business_date_index.insert(d.format("%Y-%m-%d").to_string(), i - 1);
            } else {
                self.business_dates.push(d.format("%Y-%m-%d").to_string());
                self.business_dates_index.insert(d.format("%Y-%m-%d").to_string(), i);
                i = i + 1;
            }

            d = d.succ_opt().unwrap();
        }

        self.start_date = start_date_str;
        self.end_date = end_date_str;
        self.holidays = holidays;
    }

    pub fn is_holiday(date: NaiveDate, holidays: &Vec<String>) -> bool {
        let str_date: String = date.format("%Y-%m-%d").to_string();

        return date.weekday() == Weekday::Sat || date.weekday() == Weekday::Sun || holidays.contains(&str_date)
    }

    fn range_check(&mut self, date: NaiveDate) {
        let start_date_obj = NaiveDate::parse_from_str(&self.start_date, "%Y-%m-%d").unwrap();
        let end_date_obj = NaiveDate::parse_from_str(&self.end_date, "%Y-%m-%d").unwrap();
        if (date - start_date_obj).num_days() < 0 {
            // puts "Reconstruindo calculadora de feriados pois dia #{date} eh menor que #{@start_date} -> #{@end_date}"
            let new_start_date_str = date.pred_opt().unwrap().format("%Y-%m-%d").to_string();
            self.build(new_start_date_str, self.end_date.clone(), self.holidays.clone());
        } else if (end_date_obj - date ).num_days() < 0 {
            // puts "Reconstruindo calculadora de feriados pois dia #{date} eh maior que #{end_date}"
            let new_end_date_str = date.pred_opt().unwrap().format("%Y-%m-%d").to_string();
            self.build(self.start_date.clone(), new_end_date_str, self.holidays.clone());
        }
    }

    pub fn adjust(&mut self, date: NaiveDate) -> String {
        self.range_check(date);
        let date_str: String = date.format("%Y-%m-%d").to_string();
        if !BusinessCalendar::is_holiday(date, &self.holidays) {
            return date_str;
        } else {
            return self.business_dates[*self.next_business_date_index.get(&date_str).unwrap()].clone();
        }
    }
    
    fn adjusted_date_index(&mut self, date: NaiveDate) -> &usize {
        let date_str: String = self.adjust(date).clone();
        return self.business_dates_index.get(&date_str).unwrap();
    }

    pub fn advance(&mut self, date: NaiveDate, n: i64) -> String {
        self.range_check(date);
        let index: i64 = *self.adjusted_date_index(date) as i64 + n;
        if index < 0 {
            let start_date_obj = NaiveDate::parse_from_str(&self.start_date, "%Y-%m-%d").unwrap();
            let previous_date = start_date_obj + Duration::days(n - 365);
            let new_start_date_str = previous_date.pred_opt().unwrap().format("%Y-%m-%d").to_string();
            self.build(new_start_date_str, self.end_date.clone(), self.holidays.clone());
            return self.advance(date, n)
        } else if index as usize >= self.business_dates.len() {
            let end_date_obj = NaiveDate::parse_from_str(&self.end_date, "%Y-%m-%d").unwrap();
            let next_date = end_date_obj + Duration::days(n + 365);
            let new_end_date_str = next_date.succ_opt().unwrap().format("%Y-%m-%d").to_string();
            self.build(self.start_date.clone(), new_end_date_str, self.holidays.clone());
            return self.advance(date, n)
        }
        return self.business_dates[index as usize].clone();
    }
}
