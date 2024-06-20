use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use regex::Regex;
use std::collections::HashMap;

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

#[derive(Debug)]
pub struct Schedule {
    pub events: HashMap<String, CalEvent>,
    pub comments: Vec<Comment>,
    pub done_todos: Vec<ToDo>,
    pub tbd_todos: Vec<ToDo>,
}

#[derive(Debug)]
pub struct CalEvent {
    pub time_interval: (Option<NaiveDateTime>, Option<NaiveDateTime>),
    // TODO: Figure out if description is feasible or not
    // description: String,
}

#[derive(Debug)]
pub struct Comment {
    pub time_of_write: NaiveTime,
    pub comment: String,
}

#[derive(Debug)]
pub struct ToDo {
    pub time_of_write: Option<NaiveTime>,
    pub todo: String,
    pub deadline: Option<NaiveDate>,
    pub done: bool,
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
