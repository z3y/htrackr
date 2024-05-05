use error::CliError;

mod error;
mod storage;
mod commands;
mod date;

fn main() -> Result<(), CliError> {

    let storage = storage::connect("habits.db")?;
    commands::cli(&storage)?;

    Ok(())
}
