use chrono::NaiveDate;
use chrono::Datelike;
use chrono::Weekday;
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
    pub fn is_holiday(date: NaiveDate, holidays: &Vec<String>) -> bool {
        let str_date: String = date.format("%Y-%m-%d").to_string();

        return date.weekday() == Weekday::Sat || date.weekday() == Weekday::Sun || holidays.contains(&str_date)
    }
}

impl BusinessCalendar {
    pub fn new(start_date_str: String, end_date_str: String, holidays: Vec<String>) -> BusinessCalendar {
        let mut start_date = NaiveDate::parse_from_str(&start_date_str, "%Y-%m-%d").unwrap();
        let mut end_date = NaiveDate::parse_from_str(&end_date_str, "%Y-%m-%d").unwrap();

        while BusinessCalendar::is_holiday(start_date, &holidays) {
            start_date = start_date.pred_opt().unwrap();
        }
        while BusinessCalendar::is_holiday(end_date, &holidays) {
            end_date = end_date.succ_opt().unwrap();
        }

        let mut business_dates: Vec<String> = Vec::new();
        let mut business_dates_index: HashMap<String, usize> = HashMap::new();
        let mut next_business_date_index: HashMap<String, usize> = HashMap::new();
        let mut prev_business_date_index: HashMap<String, usize> = HashMap::new();

        let mut i: usize = 0;
        let mut d = start_date.clone();

        while (end_date - d).num_days() >= 0 {
            if BusinessCalendar::is_holiday(d, &holidays) && i > 0{
                next_business_date_index.insert(d.format("%Y-%m-%d").to_string(), i);
                prev_business_date_index.insert(d.format("%Y-%m-%d").to_string(), i - 1);
            } else {
                business_dates.push(d.format("%Y-%m-%d").to_string());
                business_dates_index.insert(d.format("%Y-%m-%d").to_string(), i);
                i = i + 1;
            }

            d = d.succ_opt().unwrap();
        }

        BusinessCalendar {
            start_date: start_date_str,
            end_date: end_date_str,
            holidays: holidays,
            business_dates: business_dates,
            business_dates_index: business_dates_index,
            prev_business_date_index: prev_business_date_index,
            next_business_date_index: next_business_date_index
        }
    }

    fn is_holiday_date(&self, date: NaiveDate) -> bool {
        return BusinessCalendar::is_holiday(date, &self.holidays)
    }

    fn range_check(&self, date: NaiveDate) {
        let start_date_obj = NaiveDate::parse_from_str(&self.start_date, "%Y-%m-%d").unwrap();
        let end_date_obj = NaiveDate::parse_from_str(&self.end_date, "%Y-%m-%d").unwrap();
        if (date - start_date_obj).num_days() < 0 {
            // puts "Reconstruindo calculadora de feriados pois dia #{date} eh menor que #{@start_date} -> #{@end_date}"
            // build(date - 2.days, @end_date, @holidays)
        } else if (date - end_date_obj).num_days() > 0 {
            // puts "Reconstruindo calculadora de feriados pois dia #{date} eh maior que #{end_date}"
            // build(@start_date, date + 252.days, @holidays)
        }
    }

    fn adjust(&self, date: NaiveDate) -> String {
        self.range_check(date);
        let date_str: String = date.format("%Y-%m-%d").to_string();
        if !BusinessCalendar::is_holiday(date, &self.holidays) {
            return date_str;
        } else {
            println!("{}", date_str);
            println!("{}", *self.next_business_date_index.get(&date_str).unwrap());
            println!("{}", self.business_dates[*self.next_business_date_index.get(&date_str).unwrap()]);
            return self.business_dates[*self.next_business_date_index.get(&date_str).unwrap()].clone();
        }
    }
    
    fn adjusted_date_index(&self, date: NaiveDate) -> usize {
        println!("{}", self.adjust(date));
        return *self.business_dates_index.get(&self.adjust(date)).unwrap();
    }

    pub fn advance(&self, date: NaiveDate, n: usize) -> String {
        &self.range_check(date);
        let index: usize = self.adjusted_date_index(date) + n;
        if index < 0 {
//              build(date + (index - margin).days, @end_date, @holidays)
//            return self.advance(date, n)
        }
        return self.business_dates[self.adjusted_date_index(date) + n].clone();
    }
}
