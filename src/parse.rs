use std::{collections::HashMap, fs, iter::Peekable, path::PathBuf};

use crate::structs::{self, TimeInterval};

use anyhow::Result;
use chrono::{NaiveDate, NaiveTime};
use pulldown_cmark::{BlockQuoteKind, Event, Options, Parser, Tag, TagEnd};
use regex::Regex;

pub fn parse_sequence(
    start_date: &NaiveDate,
    end_date: &NaiveDate,
    path: &mut PathBuf,
) -> structs::Schedule {
    let mut day_iter = start_date.iter_days().peekable();

    let tbd_todos = Vec::new();
    let events = HashMap::new();
    let comments = Vec::new();

    let all_regexes = structs::init_regexes();

    let mut sched = structs::Schedule {
        events,
        comments,

        tbd_todos,
    };

    while day_iter.peek().unwrap() != end_date {
        let date = day_iter.next().unwrap();
        if parse_one_day(&date, path, &all_regexes, &mut sched).is_err() {
            path.pop();
            continue;
        }
        path.pop();
    }
    sched
}

pub fn parse_one_day(
    date: &NaiveDate,
    path: &mut PathBuf,
    all_regexes: &structs::AllRegexes,
    sched: &mut structs::Schedule,
) -> Result<()> {
    let filename = format!("{}.md", date.format("%Y-%m-%d"));
    path.push(filename);

    let contents = fs::read_to_string(&path)?;

    let mut parse_stream = Parser::new_ext(
        &contents,
        Options::ENABLE_TASKLISTS | Options::ENABLE_GFM,
    )
    .peekable();

    while let Some(content) = parse_stream.next() {
        match content {
            Event::TaskListMarker(done) if !done => parse_tasks(
                &mut sched.tbd_todos,
                &mut parse_stream,
                &all_regexes.deadline,
                &all_regexes.at_time,
                date,
            ),
            Event::Start(Tag::BlockQuote(Some(BlockQuoteKind::Note))) => {
                parse_comments(
                    date,
                    &mut sched.comments,
                    &all_regexes.at_time,
                    &mut parse_stream,
                )
            }
            Event::Start(Tag::BlockQuote(Some(BlockQuoteKind::Important))) => {
                parse_schedule(
                    &mut sched.events,
                    &all_regexes.at_time,
                    &all_regexes.end,
                    &all_regexes.all_day,
                    &mut parse_stream,
                    date,
                );
            }
            _ => continue,
        }
    }
    Ok(())
}

fn parse_schedule(
    events: &mut HashMap<String, structs::CalEvent>,
    start_search: &Regex,
    end_search: &Regex,
    all_day_search: &Regex,
    parse_stream: &mut Peekable<Parser>,
    date: &NaiveDate,
) {
    let mut content = String::new();

    if let Some(Event::Text(node)) = parse_stream.peek() {
        content.push_str(node);
    }

    let start_time = start_search.find(&content);
    if let Some(time) = start_time {
        let name = content.replace(time.as_str(), "");
        let time_interval = (
            TimeInterval((
                *date,
                Some(
                    NaiveTime::parse_from_str(
                        time.as_str().split_once(':').unwrap().1.trim(),
                        structs::TIME_FMT.fmt,
                    )
                    .unwrap_or_default(),
                ),
            )),
            TimeInterval((*date, None)),
        );
        events.insert(
            name,
            structs::CalEvent {
                start_time: time_interval.0,
                end_time: time_interval.1,
            },
        );
        return;
    }

    let all_day = all_day_search.find(&content);
    if let Some(time) = all_day {
        let name = content.replace(time.as_str(), "");
        let time_interval =
            (TimeInterval((*date, None)), TimeInterval((*date, None)));
        events.insert(
            name,
            structs::CalEvent {
                start_time: time_interval.0,
                end_time: time_interval.1,
            },
        );
        return;
    }

    let end_time = end_search.find(&content);
    if let Some(time) = end_time {
        let name = content.replace(time.as_str(), "");
        let cal_event = events.get_mut(&name).unwrap();
        cal_event.start_time =
            TimeInterval((
                cal_event.start_time.0 .0,
                Some(
                    cal_event.start_time.0 .1.unwrap_or(
                        NaiveTime::from_hms_opt(00, 00, 00).unwrap(),
                    ),
                ),
            ));
        cal_event.end_time = TimeInterval((
            *date,
            Some(
                NaiveTime::parse_from_str(
                    time.as_str().split_once(':').unwrap().1.trim(),
                    structs::TIME_FMT.fmt,
                )
                .unwrap(),
            ),
        ));
    }
}

fn parse_tasks(
    tbd: &mut Vec<structs::ToDo>,
    parse_stream: &mut Peekable<Parser>,
    deadline_search: &Regex,
    time_search: &Regex,
    date: &NaiveDate,
) {
    if let Some(Event::Text(node)) = parse_stream.peek() {
        let deadline = deadline_search.find(node);
        let time_of_write = time_search.find(node);

        let todo = node
            .replace(deadline.map_or("", |date| date.into()), "")
            .replace(time_of_write.map_or("", |time| time.into()), "");

        let deadline = deadline.map(|date| {
            NaiveDate::parse_from_str(
                date.as_str().split_once(':').unwrap().1.trim(),
                structs::DATE_FMT.fmt,
            )
            .unwrap()
        });
        let time_of_write = time_of_write.map(|time| {
            NaiveTime::parse_from_str(
                time.as_str().split_once(':').unwrap().1.trim(),
                structs::TIME_FMT.fmt,
            )
            .unwrap()
        });

        let task = structs::ToDo {
            date: *date,
            time_of_write,
            todo,
            deadline,
        };
        tbd.push(task);
    }
}

fn parse_comments(
    date: &NaiveDate,
    comments: &mut Vec<structs::Comment>,
    time_search: &Regex,
    parse_stream: &mut Peekable<Parser>,
) {
    let mut comment = String::new();
    while parse_stream.peek() != Some(&Event::End(TagEnd::BlockQuote)) {
        if let Some(Event::Text(node)) = parse_stream.next() {
            comment.push_str(&node);
            comment.push('\n');
        }
    }

    let time = time_search.find(&comment);
    if let Some(time) = time {
        let comment = comment.replace(time.as_str(), "");
        let time_of_write = date.and_time(
            NaiveTime::parse_from_str(
                time.as_str().split_once(':').unwrap().1.trim(),
                structs::TIME_FMT.fmt,
            )
            .unwrap(),
        );

        comments.push(structs::Comment {
            time_of_write,
            comment,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn check_file_parser_single() {
        let date = NaiveDate::from_ymd_opt(2024, 6, 19).unwrap();
        let all_regexes = structs::init_regexes();
        let mut sched = structs::Schedule {
            events: HashMap::new(),
            comments: Vec::new(),
            tbd_todos: Vec::new(),
        };

        assert!(parse_one_day(
            &date,
            PathBuf::from_str("tests").as_mut().unwrap(),
            &all_regexes,
            &mut sched,
        )
        .is_ok());
        println!("{:#?}", sched);
    }

    #[test]
    fn check_file_parser_range() {
        let start_date = NaiveDate::from_ymd_opt(2024, 6, 1).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2024, 6, 30).unwrap();

        let sched = parse_sequence(
            &start_date,
            &end_date,
            PathBuf::from_str("tests").as_mut().unwrap(),
        );
        println!("{:#?}", sched);
    }
}
