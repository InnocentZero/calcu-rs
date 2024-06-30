use std::collections::HashMap;

use crate::{
    config::{CommentConfig, ScheduleConfig, TodoConfig},
    structs::{CalEvent, Comment, ToDo},
};

use tabled::{
    settings::{
        formatting::{AlignmentStrategy, TrimStrategy},
        peaker::PriorityMax,
        style::HorizontalLine,
        Alignment, Format, Margin, Theme, Width,
    },
    Table,
};

macro_rules! configure_table {
    ($table:ident, $theme:ident, $config:ident, $width:ident) => {
        let alignment: Alignment = $config.alignment.into();
        $table
            .with($theme)
            .with(Margin::new(
                $config.margins.0,
                $config.margins.1,
                $config.margins.2,
                $config.margins.3,
            ))
            .with(
                Width::wrap($width as usize)
                    .keep_words()
                    .priority::<PriorityMax>(),
            )
            .with(TrimStrategy::Horizontal)
            .with(Width::increase($width as usize))
            .with(alignment)
            .with(Alignment::center_vertical())
            .with(AlignmentStrategy::PerLine);
    };
}

pub fn print_schedule(
    cal_events: &HashMap<String, CalEvent>,
    config: &ScheduleConfig,
    width: u16,
) {
    let mut theme: Theme = config.table_style.into();
    configure_theme(&mut theme);

    let mut table = Table::new(cal_events);
    table.modify((0, 0), Format::content(|_| "Schedule".to_string()));
    configure_table!(table, theme, config, width);

    println!("{table}");
}

pub fn print_todos(todos: &Vec<ToDo>, config: &TodoConfig, width: u16) {
    let mut theme: Theme = config.table_style.into();
    configure_theme(&mut theme);

    let mut table = Table::new(todos);
    configure_table!(table, theme, config, width);

    println!("{table}");
}

pub fn print_comments(
    comments: &Vec<Comment>,
    config: &CommentConfig,
    width: u16,
) {
    let mut theme: Theme = config.table_style.into();
    configure_theme(&mut theme);

    let mut table = Table::new(comments);
    configure_table!(table, theme, config, width);

    println!("{table}");
}

fn configure_theme(theme: &mut Theme) {
    let mut horizontals = HashMap::new();
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
        let start_date = NaiveDate::from_ymd_opt(2024, 6, 1).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2024, 6, 30).unwrap();

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
            80,
        );
        print_schedule(
            &sched.events,
            &ScheduleConfigBuilder::default()
                .table_style(TableStyle::Extended)
                .build()
                .unwrap(),
            80,
        );
        print_schedule(
            &sched.events,
            &ScheduleConfigBuilder::default()
                .table_style(TableStyle::Rounded)
                .build()
                .unwrap(),
            80,
        );
    }

    #[test]
    fn check_todos() {
        let start_date = NaiveDate::from_ymd_opt(2024, 6, 1).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2024, 6, 30).unwrap();

        let sched = parse_sequence(
            &start_date,
            &end_date,
            PathBuf::from_str("tests").as_mut().unwrap(),
        );

        print_todos(
            &sched.tbd_todos,
            &TodoConfigBuilder::default().build().unwrap(),
            80,
        );
        print_todos(
            &sched.tbd_todos,
            &TodoConfigBuilder::default()
                .table_style(TableStyle::Extended)
                .build()
                .unwrap(),
            80,
        );
    }

    #[test]
    fn check_comments() {
        let start_date = NaiveDate::from_ymd_opt(2024, 6, 1).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2024, 6, 30).unwrap();

        let sched = parse_sequence(
            &start_date,
            &end_date,
            PathBuf::from_str("tests").as_mut().unwrap(),
        );

        print_comments(
            &sched.comments,
            &CommentConfigBuilder::default().build().unwrap(),
            80,
        );

        print_comments(
            &sched.comments,
            &CommentConfigBuilder::default()
                .table_style(TableStyle::Extended)
                .build()
                .unwrap(),
            80,
        );
    }
}
