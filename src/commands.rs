use std::io::stdin;

use chrono::Datelike;
use clap::{arg, ArgMatches, Command};

use crate::{date::{self, Date}, error::CliError, storage::Storage};


pub fn cli(storage: &Storage) -> Result<(), CliError> {

    let matches = create_commands().get_matches();

    match matches.subcommand() {
        Some(("list", s)) => list(s, storage),
        Some(("create", s)) => create(s, storage),
        Some(("delete", s)) => delete(s, storage),
        Some(("rename", s)) => rename(s, storage),
        Some(("id", s)) => id(s, storage),
        Some(("mark", s)) => mark(s, storage),
        Some(("unmark", s)) => unmark(s, storage),

        _ => Err(CliError::new("invalid command"))
    }
}

fn create_commands() -> Command {

    let short_date_help = "Optional date in YYYY-MM format";
    let date_help = "Date in YYYY-MM-DD format, or yesterday (y)";


    Command::new("htrackr")
    .arg_required_else_help(true)
        .subcommand(Command::new("list")
            .about("List habits for month")
                .arg(arg!(-c --compact "Compact print")
                .required(false)
            )
            .arg(arg!(date: [DATE]).required(false).help(short_date_help))
        )
        .subcommand(Command::new("create")
            .about("Create new habit")
            .arg(arg!(name: [NAME]))
            .arg_required_else_help(true)
        )
        .subcommand(Command::new("delete")
            .about("Delete habit")
            .arg(arg!(name: [NAME]))
            .arg_required_else_help(true)
        )
        .subcommand(Command::new("rename")
            .about("Rename habit")
            .arg(arg!(name: [NAME]))
            .arg(arg!(new_name: [NEW_NAME]))
        )
        .subcommand(Command::new("id")
            .arg(arg!(name: [NAME]))
            .about("Get ID")
        )
        .subcommand(Command::new("mark")
            .about("Mark habit as complete for date")
            .arg(arg!(name: [NAME]))
            .arg_required_else_help(true)
            .arg(arg!(date: [DATE]).required(false).help(date_help))
        )
        .subcommand(Command::new("unmark")
            .about("Unmark habit as complete for date")
            .arg(arg!(name: [NAME]))
            .arg_required_else_help(true)
            .arg(arg!(date: [DATE]).required(false).help(date_help))
        )
}

fn list(matches: &ArgMatches, storage: &Storage) -> Result<(), CliError> {

    let list = storage.habit_list()?;
    // let compact = matches.contains_id("compact");
    let local = chrono::Local::now();

    let year;
    let month;

    if let Some(date) = matches.get_one::<String>("date") {
        let mut full_date = date.clone();
        full_date.push_str("-01");
        let date = Date::from_string(&full_date)?;
        year = date.year;
        month = date.month;
    } else {
        year = local.year();
        month = local.month() as i32;
    }
    

    let num_days = date::num_days(year, month);

    let date_start = Date {
        year: year,
        month: month,
        day: 01,
    };
    let date_end = Date {
        year: year,
        month: month,
        day: num_days,
    };

    let month_display = format!("{:04}-{:02}", year, month);

    let mut target_indent = month_display.len() + 2;
    for name in &list {
        let len = name.len();
        if len > target_indent {
            target_indent = len;
        }
    }

    let mut line0 = String::new();
    line0.push_str(&month_display);
    line0.push_str(&str::repeat(" ", target_indent - month_display.len()));
    line0.push_str("| ");
    for i in 1..num_days+1 {
        line0.push_str(&format!("{}", i % 10));
    }
    println!("{}", line0);

    for name in &list {
        let days = storage.get_marked_days(&name, &date_start, &date_end);
        match days {
            Ok(days) =>{
                let indent_count = target_indent - name.len();
                let indent = str::repeat(" ", indent_count);
                
                let cap = name.len() + indent.len() + 1 + num_days as usize + 1;
                let mut line = String::with_capacity(cap);
                line.push_str(&name);
                line.push_str(&indent);
                line.push_str("| ");

                for i in 1..num_days+1 {
                    match days.iter().any(|f| f.day == i) {
                        true => line.push_str("X"),
                        false => line.push_str(" "),
                    }
                }

                // if cap != line.len() {
                //     panic!("capacity")
                // }
                
                println!("{}", line);
            },
            Err(e) => println!("error {}", e),
        };
    }

    Ok(())
}

fn create(matches: &ArgMatches, storage: &Storage) -> Result<(), CliError> {

    if let Some(name) = matches.get_one::<String>("name") {
        storage.create_habit(name)?;
    } else {
        return Err(CliError::new("name is required"));
    }

    Ok(())
}

fn delete(matches: &ArgMatches, storage: &Storage) -> Result<(), CliError> {

    
    if let Some(name) = matches.get_one::<String>("name") {
        println!("Delete habit {} and all entires? y/n", name);
        let mut line = String::with_capacity(1);
        match stdin().read_line(&mut line) {
            Ok(_) => {
                if line.len() > 1 && line.as_bytes()[0] == b'y' {
                    storage.delete_habit(name)?;
                }
            },
            Err(err) => return Err(CliError(err.to_string())),
        }
        return  Ok(());
    }

    return Err(CliError::new("invalid args"));
}

fn rename(matches: &ArgMatches, storage: &Storage) -> Result<(), CliError> {

    if let Some(name) = matches.get_one::<String>("name") {
        if let Some(new_name) = matches.get_one::<String>("new_name") {
            storage.rename_habit(&name, &new_name)?;

            return Ok(());
        }
    }

    return Err(CliError::new("invalid args"));
}

fn id(matches: &ArgMatches, storage: &Storage) -> Result<(), CliError> {

    if let Some(name) = matches.get_one::<String>("name") {
        let id = storage.get_habit_id(&name)?;
        println!("{}", id);
        return Ok(());
    }

    return Err(CliError::new("invalid args"));
}

fn parse_date_arg(date: &str) -> Result<Date, CliError> {
    if date == "yesterday" || date == "y" {
        return Ok(Date::yesterday());
    }
    
    Date::from_string(date)
}

fn mark(matches: &ArgMatches, storage: &Storage) -> Result<(), CliError> {

    if let Some(name) = matches.get_one::<String>("name") {
        if let Some(date) = matches.get_one::<String>("date") {
            storage.mark_habit(&name, &parse_date_arg(&date)?)?;
            return Ok(());
        } else {
            let today = Date::today();
            storage.mark_habit(&name, &today)?;
            return Ok(());
        }
    }

    return Err(CliError::new("invalid args"));
}

fn unmark(matches: &ArgMatches, storage: &Storage) -> Result<(), CliError> {

    if let Some(name) = matches.get_one::<String>("name") {
        if let Some(date) = matches.get_one::<String>("date") {
            storage.unmark_habit(&name, &parse_date_arg(&date)?)?;
            return Ok(());
        } else {
            let today = Date::today();
            storage.unmark_habit(&name, &today)?;
            return Ok(());
        }

    }

    return Err(CliError::new("invalid args"));
}