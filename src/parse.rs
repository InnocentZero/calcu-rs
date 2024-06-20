use crate::structs;
use anyhow::Result;
use chrono::{NaiveDate, NaiveTime};
use pulldown_cmark::{BlockQuoteKind, Event, Options, Parser, Tag, TagEnd};
use regex::Regex;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
    iter::Peekable,
    path::PathBuf,
};





}

pub fn parse(date: &NaiveDate, path: &mut PathBuf) -> Result<Schedule> {
    let filename = format!("{}.md", date.format("%Y_%m_%d"));
    path.push(filename);
    let file = File::open(path)?;

    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    let mut deadline = r"DEADLINE: ".to_string();
    deadline.push_str(DATE_FMT.re);
    let deadline_search = Regex::new(&deadline).unwrap();

    let mut time = r"AT: ".to_string();
    time.push_str(TIME_FMT.re);
    let time_search = Regex::new(&time).unwrap();

    let mut end = r"END: ".to_string();
    end.push_str(TIME_FMT.re);
    let end_search = Regex::new(&end).unwrap();

    let all_day_search = Regex::new(r"ALL DAY").unwrap();

    let mut todos = Vec::new();
    let mut events = HashMap::new();
    let mut comments = Vec::new();

    let mut parse_stream = Parser::new_ext(
        &contents,
        Options::ENABLE_TASKLISTS | Options::ENABLE_GFM,
    )
    .peekable();

    while let Some(content) = parse_stream.next() {
        match content {
            Event::TaskListMarker(done) => parse_tasks(
                &mut todos,
                done,
                &mut parse_stream,
                &deadline_search,
                &time_search,
            ),
            Event::Start(Tag::BlockQuote(Some(BlockQuoteKind::Note))) => {
                parse_comments(&mut comments, &time_search, &mut parse_stream)
            }
            Event::Start(Tag::BlockQuote(Some(BlockQuoteKind::Important))) => {
                parse_schedule(
                    &mut events,
                    &time_search,
                    &end_search,
                    &all_day_search,
                    &mut parse_stream,
                );
            }
            _ => continue,
        }
    }
    Ok(Schedule {
        events,
        comments,
        todos,
    })
}

fn parse_schedule(
    events: &mut HashMap<String, structs::CalEvent>,
    start_search: &Regex,
    end_search: &Regex,
    all_day_search: &Regex,
    parse_stream: &mut Peekable<Parser>,
) {
    let mut content = String::new();

    while parse_stream.peek() != Some(&Event::End(TagEnd::BlockQuote)) {
        if let Some(Event::Text(node)) = parse_stream.next() {
            content.push_str(&node);
            content.push('\n');
        }
    }

    let start_time = start_search.find(&content);
    if let Some(time) = start_time {
        let name = content.replace(time.as_str(), "");
        let time_interval = (
            Some(
                NaiveTime::parse_from_str(
                    time.as_str().split_once(':').unwrap().1.trim(),
                    "%I:%M %p",
                )
                .unwrap_or_default(),
            ),
            None,
        );
        events.insert(name, CalEvent { time_interval });
        return;
    }

    let all_day = all_day_search.find(&content);
    if let Some(time) = all_day {
        let name = content.replace(time.as_str(), "");
        let time_interval = (None, None);
        events.insert(name, CalEvent { time_interval });
        return;
    }

    let end_time = end_search.find(&content);
    if let Some(time) = end_time {
        let name = content.replace(time.as_str(), "");
        let cal_event = events.get_mut(&name).unwrap();
        cal_event.time_interval = (
            Some(
                cal_event
                    .time_interval
                    .0
                    .unwrap_or(NaiveTime::from_hms_opt(00, 00, 00).unwrap()),
            ),
            NaiveTime::parse_from_str(
                time.as_str().split_once(':').unwrap().1.trim(),
                structs::TIME_FMT.fmt,
            )
            .ok(),
        );
    }
}

fn parse_tasks(
    todos: &mut Vec<structs::ToDo>,
    tbd: &mut Vec<structs::ToDo>,
    done: bool,
    parse_stream: &mut Peekable<Parser>,
    deadline_search: &Regex,
    time_search: &Regex,
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
            time_of_write,
            todo,
            deadline,
            done,
        };
        if !done {
            tbd.push(task);
        } else {
            todos.push(task);
        }
    }
}

fn parse_comments(
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
        let time_of_write = NaiveTime::parse_from_str(
            time.as_str().split_once(':').unwrap().1.trim(),
            structs::TIME_FMT.fmt,
        )
        .unwrap();

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
    fn check_file_parser() {
        let date = NaiveDate::from_ymd_opt(2024, 6, 19).unwrap();
        let sched = parse(&date, PathBuf::from_str("tests").as_mut().unwrap());
        println!("{:?}", sched.unwrap());
    }
}
