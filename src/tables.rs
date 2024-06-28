use std::collections::HashMap;

use tabled::{
    builder::Builder,
    settings::{style::HorizontalLine, Alignment, Margin, Theme},
    Table,
};

use crate::{
    config::{CommentConfig, ScheduleConfig, TodoConfig},
    structs::{CalEvent, Comment, ToDo},
};

pub fn print_schedule(
    cal_events: &HashMap<String, CalEvent>,
    config: &ScheduleConfig,
) {
    let mut builder = Builder::new();
    builder.push_record(["Schedule", "Start Day and Time", "End Day and Time"]);
    for (key, value) in cal_events {
        builder.push_record([
            key,
            &format!("{}", value.start_time),
            &format!("{}", value.end_time),
        ]);
    }
    let mut table = builder.build();

    let mut horizontals = HashMap::new();

    let mut theme: Theme = config.table_style.into();
    if let Some(hor) = theme.get_border_horizontal() {
        horizontals.insert(1, HorizontalLine::new(hor).into_inner());
    }
    theme.remove_border_horizontal();
    theme.set_lines_horizontal(horizontals);
    if let Some(chr) = theme.get_border_vertical() {
        theme.set_border_intersection(chr);
        theme.set_border_intersection_left(chr);
        theme.set_border_intersection_right(chr);
    }
    theme.align_columns(Alignment::center_vertical());

    let alignment: Alignment = config.alignment.into();

    table
        .with(theme)
        .with(Margin::new(
            config.margins.0,
            config.margins.1,
            config.margins.2,
            config.margins.3,
        ))
        .with(alignment);

    println!("{table}");
}

pub fn print_todos(todos: &Vec<ToDo>, config: &TodoConfig) {
    let mut horizontals = HashMap::new();

    let mut theme: Theme = config.table_style.into();
    if let Some(hor) = theme.get_border_horizontal() {
        horizontals.insert(1, HorizontalLine::new(hor).into_inner());
    }
    theme.remove_border_horizontal();
    theme.set_lines_horizontal(horizontals);
    if let Some(chr) = theme.get_border_vertical() {
        theme.set_border_intersection(chr);
        theme.set_border_intersection_left(chr);
        theme.set_border_intersection_right(chr);
    }
    theme.align_columns(Alignment::center_vertical());

    let alignment: Alignment = config.alignment.into();

    let mut table = Table::new(todos);
    table
        .with(theme)
        .with(Margin::new(
            config.margins.0,
            config.margins.1,
            config.margins.2,
            config.margins.3,
        ))
        .with(alignment);

    println!("{table}");
}

pub fn print_comments(comments: &Vec<Comment>, config: &CommentConfig) {
    let mut horizontals = HashMap::new();

    let mut theme: Theme = config.table_style.into();
    if let Some(hor) = theme.get_border_horizontal() {
        horizontals.insert(1, HorizontalLine::new(hor).into_inner());
    }
    theme.remove_border_horizontal();
    theme.set_lines_horizontal(horizontals);
    if let Some(chr) = theme.get_border_vertical() {
        theme.set_border_intersection(chr);
        theme.set_border_intersection_left(chr);
        theme.set_border_intersection_right(chr);
    }
    theme.align_columns(Alignment::center_vertical());

    let alignment: Alignment = config.alignment.into();

    let mut table = Table::new(comments);
    table
        .with(theme)
        .with(Margin::new(
            config.margins.0,
            config.margins.1,
            config.margins.2,
            config.margins.3,
        ))
        .with(alignment)
        .with(Alignment::center_vertical());

    println!("{table}");
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::config::{
        CommentConfigBuilder, ScheduleConfigBuilder, TableStyle,
        TodoConfigBuilder,
    };
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

        print_schedule(
            &sched.events,
            &ScheduleConfigBuilder::default()
                .table_style(TableStyle::Empty)
                .build()
                .unwrap(),
        );
        print_schedule(
            &sched.events,
            &ScheduleConfigBuilder::default()
                .table_style(TableStyle::Extended)
                .build()
                .unwrap(),
        );
        print_schedule(
            &sched.events,
            &ScheduleConfigBuilder::default()
                .table_style(TableStyle::Rounded)
                .build()
                .unwrap(),
        );
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

        print_todos(
            &sched.tbd_todos,
            &TodoConfigBuilder::default().build().unwrap(),
        );
        print_todos(
            &sched.tbd_todos,
            &TodoConfigBuilder::default()
                .table_style(TableStyle::Extended)
                .build()
                .unwrap(),
        );
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

        print_comments(
            &sched.comments,
            &CommentConfigBuilder::default().build().unwrap(),
        );

        print_comments(
            &sched.comments,
            &CommentConfigBuilder::default()
                .table_style(TableStyle::Extended)
                .build()
                .unwrap(),
        );
    }
}
