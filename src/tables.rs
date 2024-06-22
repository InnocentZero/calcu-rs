use std::collections::HashMap;

use tabled::{
    settings::{style::HorizontalLine, Alignment, Margin, Style},
    Table,
};

use crate::structs::{CalEvent, Comment, ToDo};

pub fn print_schedule(cal_events: &HashMap<String, CalEvent>) {
    let mut table = Table::new(cal_events);
    table
        .with(
            Style::extended()
                .horizontals([(1, HorizontalLine::inherit(Style::extended()))])
                .remove_horizontal(),
        )
        .with(Margin::new(4, 4, 2, 2))
        .with(Alignment::center_vertical());
    println!("{table}");
}

pub fn print_todos(todos: &Vec<ToDo>) {
    let mut table = Table::new(todos);

    table
        .with(
            Style::extended()
                .horizontals([(1, HorizontalLine::inherit(Style::extended()))])
                .remove_horizontal(),
        )
        .with(Margin::new(4, 4, 2, 2))
        .with(Alignment::center_vertical());
    println!("{table}");
}

pub fn print_comments(comments: &Vec<Comment>) {
    let mut table = Table::new(comments);

    table
        .with(
            Style::extended()
                .horizontals([(1, HorizontalLine::inherit(Style::extended()))])
                .remove_horizontal(),
        )
        .with(Margin::new(4, 4, 2, 2))
        .with(Alignment::center_vertical());
    println!("{table}");
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parse::*;
    use chrono::NaiveDate;
    use std::path::PathBuf;
    use std::str::FromStr;

    #[test]
    fn check_events() {
        let start_date = NaiveDate::from_ymd_opt(2024, 6, 19).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2024, 6, 24).unwrap();

        let sched = parse_sequence(
            &start_date,
            &end_date,
            PathBuf::from_str("tests").as_mut().unwrap(),
        );

        print_schedule(&sched.events);
    }

    #[test]
    fn check_todos() {
        let start_date = NaiveDate::from_ymd_opt(2024, 6, 19).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2024, 6, 24).unwrap();

        let sched = parse_sequence(
            &start_date,
            &end_date,
            PathBuf::from_str("tests").as_mut().unwrap(),
        );

        print_todos(&sched.tbd_todos);
        print_todos(&sched.done_todos);
    }

    #[test]
    fn check_comments() {
        let start_date = NaiveDate::from_ymd_opt(2024, 6, 19).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2024, 6, 24).unwrap();

        let sched = parse_sequence(
            &start_date,
            &end_date,
            PathBuf::from_str("tests").as_mut().unwrap(),
        );

        print_comments(&sched.comments);
    }
}
