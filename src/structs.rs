use std::{collections::HashMap, fmt::Display, ops::Deref};

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use regex::Regex;
use tabled::Tabled;

#[derive(Debug)]
pub struct TimeInterval(pub (NaiveDate, Option<NaiveTime>));

impl Display for TimeInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - ", self.0 .0)?;
        match self.0 .1 {
            Some(time) => write!(f, "{}", time.format(TIME_FMT.fmt)),
            None => write!(f, "All day"),
        }?;
        Ok(())
    }
}

impl Deref for TimeInterval {
    type Target = (NaiveDate, Option<NaiveTime>);
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct AllRegexes {
    pub deadline: Regex,
    pub at_time: Regex,
    pub end: Regex,
    pub all_day: Regex,
}

pub struct Format {
    pub re: &'static str,
    pub fmt: &'static str,
}

pub const TIME_FMT: Format = Format {
    re: "[0-9]{2}:[0-9]{2} [AP]M",
    fmt: "%I:%M %p",
};

pub const DATE_FMT: Format = Format {
    re: "[0-9]{4}-[0-9]{2}-[0-9]{2}",
    fmt: "%Y-%m-%d",
};

pub const DATETIME_FMT: Format = Format {
    re: "[0-9]{4}-[0-9]{2}-[0-9]{2} [0-9]{2}:[0-9]{2} [AP]M",
    fmt: "%Y-%m-%d %I:%M %p",
};

#[derive(Debug)]
pub struct Schedule {
    pub events: HashMap<String, CalEvent>,
    pub comments: Vec<Comment>,
    pub tbd_todos: Vec<ToDo>,
}

#[derive(Debug, Tabled)]
pub struct CalEvent {
    #[tabled(rename = "Start Time")]
    pub start_time: TimeInterval,
    #[tabled(rename = "End Time")]
    pub end_time: TimeInterval,
    // TODO: Figure out if description is feasible or not
    // description: String,
}

#[derive(Debug, Tabled)]
pub struct Comment {
    #[tabled(rename = "Time of Write", display_with = "display_datetime")]
    pub time_of_write: NaiveDateTime,
    #[tabled(rename = "Logs")]
    pub comment: String,
}

fn display_datetime(datetime: &NaiveDateTime) -> String {
    datetime.format(DATETIME_FMT.fmt).to_string()
}

#[derive(Debug, Tabled)]
pub struct ToDo {
    #[tabled(rename = "Date")]
    pub date: NaiveDate,
    #[tabled(rename = "Time of Write", display_with = "display_tow")]
    pub time_of_write: Option<NaiveTime>,
    #[tabled(rename = "ToDo")]
    pub todo: String,
    #[tabled(rename = "Deadline", display_with = "display_deadline")]
    pub deadline: Option<NaiveDate>,
}

fn display_tow(tow: &Option<NaiveTime>) -> String {
    match tow {
        Some(time) => time.format(TIME_FMT.fmt).to_string(),
        None => "None".to_string(),
    }
}

fn display_deadline(tow: &Option<NaiveDate>) -> String {
    match tow {
        Some(time) => format!("{time}"),
        None => "None".to_string(),
    }
}

pub fn init_regexes() -> AllRegexes {
    let mut deadline = r"DEADLINE: ".to_string();
    deadline.push_str(DATE_FMT.re);
    let deadline = Regex::new(&deadline).unwrap();

    let mut at_time = r"AT: ".to_string();
    at_time.push_str(TIME_FMT.re);
    let at_time = Regex::new(&at_time).unwrap();

    let mut end = r"END: ".to_string();
    end.push_str(TIME_FMT.re);
    let end = Regex::new(&end).unwrap();

    let all_day = Regex::new(r"ALL DAY").unwrap();

    AllRegexes {
        deadline,
        at_time,
        end,
        all_day,
    }
}
