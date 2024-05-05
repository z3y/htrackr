use chrono::Datelike;

use crate::error::CliError;

#[derive(Debug)]
pub struct Date {
    pub year: i32,
    pub month: i32,
    pub day: i32
}

impl Date {
    pub fn from_string(date: &str) -> Result<Date, CliError> {

        let parts = date.trim().splitn(3, "-").collect::<Vec<&str>>();

        if parts.len() != 3 {
            return Err(CliError(format!("failed to parse date {}, expected YYYY-MM-DD format", date)));
        }

        let y_str = parts[0];
        let m_str = parts[1];
        let d_str = parts[2];

        if y_str.len() != 4 {
            return Err(CliError(format!("failed to parse year {}, expected YYYY", y_str)));
        }
        if m_str.len() != 2 {
            return Err(CliError(format!("failed to parse month {}, expected MM", m_str)));
        }
        if d_str.len() != 2 {
            return Err(CliError(format!("failed to parse day {}, expected DD", d_str)));
        }

        let y = y_str.parse::<i32>()?;
        let m = m_str.parse::<i32>()?;
        let d = d_str.parse::<i32>()?;

        let result = Date {
            year: y,
            month: m,
            day: d,
        };

        if result.is_valid() {
            Ok(result)
        } else {
            return Err(CliError(format!("invalid date {}", date)));
        }
    }

    pub fn is_valid(&self) -> bool {
        let m = self.month;
        let d = self.day;
        let y = self.year;

        if y < 1 {
            return false;
        }

        if m < 1 || m > 12 {
            return false;
        }

        let last_day = num_days(y, m);
        if d < 1 || d > last_day {
            return false;
        }

        true
    }

    pub fn to_string(&self) -> Result<String, CliError> {

        let result = format!("{:04}-{:02}-{:02}", self.year, self.month, self.day);

        if !self.is_valid() {
            return Err(CliError(format!("invalid date {}", result)));
        }

        Ok(result)
    }

    pub fn today() -> Date {
        let local = chrono::Local::now();

        let year = local.year();
        let month = local.month() as i32;
        let day = local.day() as i32;

        Date {
            year,
            month,
            day: day,
        }
    }

}

pub fn num_days(year: i32, month: i32) -> i32  {

    let leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
    
    match month {
        1 => 31,
        2 => if leap {29} else {28},
        3 => 31, 
        4 => 30, 
        5 => 31, 
        6 => 30, 
        7 => 31, 
        8 => 31, 
        9 => 30, 
        10 => 31,
        11 => 30,
        12 => 31,
        _ => 0
    }
}