use rusqlite::{params, Connection};
use uuid::Uuid;

use crate::{date::Date, error::CliError};


pub struct Storage {
    conn: Connection,
}

impl Storage {

    fn initialize(&self) -> Result<(), CliError> {
        let _ = self.conn.execute(
            "
			create table if not exists habits(
			id varchar(255) primary key,
			name varchar(255)
			)",
            [])?;


        let _ = self.conn.execute(
            "
            create table if not exists habit_entries(
            habit_id varchar(255),
            date DATE,
            foreign key (habit_id) references habits(id)
            )",
            [])?;

        Ok(())
    }

    pub fn create_habit(&self, name: &str) -> Result<(), CliError> {

        if self.habit_exists(name)? {
            return Err(CliError::new("habit already exists"));
        }

        if name == "" {
            return Err(CliError::new("invaid name"));
        }

        let mut id = "hbt_".to_owned();
        id.push_str(&Uuid::new_v4().to_string());

        let _ = self.conn.execute(
            "
            insert into habits
            (id, name)
            values (?1, ?2)
            ",
            params![id, name])?;

        Ok(())
    }

    pub fn delete_habit(&self, name: &str) -> Result<(), CliError> {

        if !self.habit_exists(name)? {
            return Err(CliError(format!("habit {} not found", name)));
        }

        let _ = self.conn.execute("delete from habits where name = ?1", params![name])?;

        Ok(())
    }

    pub fn rename_habit(&self, name: &str, new_name: &str) -> Result<(), CliError> {

        if !self.habit_exists(name)? {
            return Err(CliError(format!("habit {} not found", name)));
        }

        let _ = self.conn.execute("update habits set name = ?1 where name = ?2", params![new_name, name])?;

        Ok(())
    }

    pub fn habit_exists(&self, name: &str) -> Result<bool, CliError> {

        let result: i32 = self.conn.query_row("select count(1) from habits where name = ?1",
        params![name],
        |row| row.get(0))?;

        Ok(result > 0)
    }

    pub fn habit_list(&self) -> Result<Vec<String>, CliError> {

        let mut stmt = self.conn.prepare("select name from habits")?;

        let string_iter = stmt.query_map([], |row| {
            let column: String = row.get(0)?;
            Ok(column)
        })?;

        let mut result: Vec<String> = vec![];

        for string_result in string_iter {
            let string_value: String = string_result?;
            result.push(string_value)
        }

        Ok(result)
    }

    pub fn get_habit_id(&self, name: &str) -> Result<String, CliError> {

        let result: Result<String, rusqlite::Error> = self.conn.query_row("select id from habits where name = ?1",
        params![name],
        |row| row.get(0));

        match result {
            Ok(r) => Ok(r),
            Err(_) => Err(CliError(format!("habit {} not found", name))),
        }
    }

    pub fn mark_habit(&self, name: &str, date: &Date) -> Result<(), CliError> {
        let date = date.to_string()?;

        let id = self.get_habit_id(name)?;

        let result: i32 = self.conn.query_row("select count(1) from habit_entries where habit_id = ?1 and date = ?2",
        params![id, date],
        |row| row.get(0))?;

        if result > 0 {
            return Err(CliError(format!("habit {} already marked for {} date", name, date)));
        }

        self.conn.execute("insert into habit_entries (habit_id, date) values (?1, ?2)", params![id, date])?;

        Ok(())
    }

    pub fn unmark_habit(&self, name: &str, date: &Date) -> Result<(), CliError> {

        let date = date.to_string()?;
        let id = self.get_habit_id(name)?;

        let result: i32 = self.conn.query_row("select count(1) from habit_entries where habit_id = ?1 and date = ?2",
        params![id, date],
        |row| row.get(0))?;

        if result == 0 {
            return Err(CliError(format!("habit {} is not marked for {} date", name, date)));
        }

        self.conn.execute("delete from habit_entries where habit_id = ?1 and date = ?2", params![id, date])?;

        Ok(())
    }

    pub fn get_marked_days(&self, name: &str, date_start: &Date, date_end: &Date) -> Result<Vec<Date>, CliError> {

        let date_start = date_start.to_string()?;
        let date_end = date_end.to_string()?;

        let id = self.get_habit_id(name)?;

        let mut stmt = self.conn.prepare("select date from habit_entries where habit_id = ?1 and date between ?2 and ?3")?;

        let string_iter = stmt.query_map(params![id, date_start, date_end], |row| {
            let column: String = row.get(0)?;
            Ok(column)
        })?;

        let mut result: Vec<Date> = vec![];
        for string_result in string_iter {
            let string_value: String = string_result?;
            let parsed = Date::from_string(&string_value);
            match parsed {
                Ok(r) => result.push(r),
                Err(_) => (),
            };
        }

        Ok(result)
    }

}

pub fn connect(path: &str) -> Result<Storage, CliError> {
    let conn = Connection::open(path);

    let storage = Storage {
        conn: conn.expect("failed to initialize storage"),
    };

    storage.initialize()?;

    Ok(storage)
}