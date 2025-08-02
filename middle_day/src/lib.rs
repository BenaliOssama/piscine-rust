use chrono::{NaiveDate, Datelike};

pub fn middle_day(year: u32) -> Option<chrono::Weekday> {

    let is_leap = NaiveDate::from_ymd_opt(year as i32, 2, 29).is_some();

    let middle_day = if is_leap{ 
        return None
    } else { 
        183
    };

    //-------------------> year + ordinal day
    let middle = NaiveDate::from_yo_opt(year as i32, middle_day).unwrap();
    
    Some(middle.weekday())
}
