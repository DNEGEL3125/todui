use anyhow::Result;
use clap::Parser;

use crate::app::App;
use crate::cli::cli_utils;
use crate::cli::formats::Format;
use crate::task::Task;
use crate::{repeat, utils};

#[derive(Parser)]
pub struct Args {
    /// The name of the new task
    name: String,
    /// The date the task is due
    #[arg(long)]
    date: Option<String>,
    /// How often the task repeats
    #[arg(long)]
    repeats: Option<String>,
    /// The group the task belongs to
    #[arg(long)]
    group: Option<String>,
    /// A description or url for your task
    #[arg(long)]
    description: Option<String>,
    /// A url for your task
    #[arg(long)]
    url: Option<String>,
    /// The format to display the new task with
    #[arg(long)]
    format: Option<Format>,
}

pub fn run(mut app: App, args: Args) -> Result<()> {
    let Args {
        name,
        format,
        date,
        repeats,
        group,
        description,
        url,
    } = args;

    let mut task = Task::default();
    task.set_name(name);
    let date =
        utils::parse_date(&date.unwrap_or_default(), &app.settings).unwrap_or(utils::get_today());
    task.set_date(date);
    task.set_repeats(
        repeat::Repeat::parse_from_str(&repeats.unwrap_or_default()).unwrap_or_default(),
    );
    task.set_group(group.unwrap_or_default());
    task.set_description(description.unwrap_or_default());
    task.set_url(url.unwrap_or_default());

    let id = app.add_task(task);
    let task = app.get_task(id).unwrap();
    cli_utils::print_task(task, format, &app.settings);

    Ok(())
}
