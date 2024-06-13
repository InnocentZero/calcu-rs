use anyhow::Result;
use chrono::{NaiveDate, NaiveTime};
use pulldown_cmark::{BlockQuoteKind, Event, Options, Parser, Tag, TagEnd};
use regex::Regex;
use std::{
    fs::File,
    io::{BufReader, Read},
    iter::Peekable,
    path::PathBuf,
};

pub struct Schedule {
    events: Vec<CalEvent>,
    comments: Vec<Comment>,
    todos: Vec<ToDo>,
}

pub struct CalEvent {
    time_interval: Option<(NaiveTime, NaiveTime)>,
    name: String,
    description: String,
}

pub struct Comment {
    time_of_write: NaiveTime,
    comment: String,
}

pub struct ToDo {
    time_of_write: Option<NaiveTime>,
    todo: String,
    deadline: Option<NaiveDate>,
    done: bool,
}

pub fn parse(date: &NaiveDate, path: &mut PathBuf) -> Result<Schedule> {
    let filename = format!("{}.md", date.format("%Y_%m_%d"));
    path.push(filename);
    let file = File::open(path)?;

    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    let deadline_search = Regex::new(r"DEADLINE: [0-9]{4}-[0-9]{2}-[0-9]{2}").unwrap();
    let time_search = Regex::new(r"[0-9]{2}:[0-9]{2} [AP]M").unwrap();

    let start_search = Regex::new(r"START: [0-9]{2}:[0-9]{2} [AP]M").unwrap();
    let end_search = Regex::new(r"END: [0-9]{2}:[0-9]{2} [AP]M").unwrap();

    let mut tasks = Vec::new();
    // let mut events = Vec::new();
    let mut comments = Vec::new();

    let mut parse_stream =
        Parser::new_ext(&contents, Options::ENABLE_TASKLISTS | Options::ENABLE_GFM).peekable();

    while let Some(content) = parse_stream.next() {
        match content {
            Event::TaskListMarker(done) => parse_tasks(
                &mut tasks,
                done,
                &mut parse_stream,
                &deadline_search,
                &time_search,
            ),
            Event::Start(Tag::BlockQuote(Some(BlockQuoteKind::Note))) => {
                parse_comments(&mut comments, &time_search, &mut parse_stream)
            }
            // TODO: decide on the syntax for Schedules
            // Event::
            _ => todo!(),
        }
    }
    todo!()
}

fn parse_tasks(
    tasks: &mut Vec<ToDo>,
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

        let deadline =
            deadline.map(|date| NaiveDate::parse_from_str(date.into(), "%Y-%m-%d").unwrap());
        let time_of_write =
            time_of_write.map(|time| NaiveTime::parse_from_str(time.into(), "%Y-%m-%d").unwrap());

        tasks.push(ToDo {
            time_of_write,
            todo,
            deadline,
            done,
        });
    }
}

fn parse_comments(
    comments: &mut Vec<Comment>,
    time_search: &Regex,
    parse_stream: &mut Peekable<Parser>,
) {
    let mut comment = String::new();
    while parse_stream.peek() != Some(&Event::End(TagEnd::BlockQuote)) {
        if let Some(Event::Text(node)) = parse_stream.next() {
            comment.push_str(&node);
        }
    }

    let time = time_search.find(&comment);
    if let Some(time) = time {
        let comment = comment.replace(time.as_str(), "");
        let time_of_write = NaiveTime::parse_from_str(time.into(), "%Y-%m-%d").unwrap();

        comments.push(Comment {
            time_of_write,
            comment,
        })
    }
}
