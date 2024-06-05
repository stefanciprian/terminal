//#[macro_use]
//extern crate diesel;

use crossterm::event::{self, Event, KeyCode};
use crossterm::style::{PrintStyledContent, Stylize};
use crossterm::ExecutableCommand;
use diesel::prelude::*;
use diesel::sqlite::{Sqlite, SqliteConnection};
use dotenvy::dotenv;
use std::env;
use std::io::{stdout, Write};

#[derive(Queryable, Insertable, Selectable, Debug)]
#[diesel(table_name = env_vars)]
#[diesel(check_for_backend(Sqlite))]
struct EnvVar {
    id: Option<i32>,
    key: String,
    value: String,
}

fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

table! {
    env_vars (id) {
        id -> Nullable<Integer>,
        key -> Text,
        value -> Text,
    }
}

pub fn list_env_command() {
    let mut connection = establish_connection();

    let results = env_vars::table
        .select(EnvVar::as_select()) // Use the Selectable trait to match struct fields
        .load::<EnvVar>(&mut connection) // Pass mutable reference
        .expect("Error loading env vars");

    println!("Displaying {} env vars", results.len());
    for env_var in results {
        println!("{}: {}", env_var.key, env_var.value);
    }
}

pub fn set_env_command(input_buffer: String) {
    let mut stdout = stdout();
    let greeting = "Setting environment variables".green();
    stdout.execute(PrintStyledContent(greeting)).unwrap();
    stdout.flush().unwrap();

    let mut connection = establish_connection();

    // Load environment variables from .env file
    dotenv().ok();

    // Extract the key and value from the input_buffer
    let key_value = input_buffer.trim_start_matches("set env").trim();
    let key_value: Vec<&str> = key_value.split_whitespace().collect();
    if key_value.len() != 2 {
        eprintln!("Invalid command format. Use: set env <KEY> <VALUE>");
        return;
    }

    let key = key_value[0];
    let value = key_value[1];

    // Check if the key exists
    use self::env_vars::dsl::{env_vars, key as env_key, value as env_value};

    let existing_var = env_vars
        .filter(env_key.eq(&key))
        .first::<EnvVar>(&mut connection)
        .optional()
        .expect("Error loading env var");

    if let Some(_) = existing_var {
        // Prompt user for confirmation to update
        let prompt_message = format!(
            "The key '{}' already exists. Do you want to update it? (y/n): ",
            key
        )
        .yellow();
        stdout.execute(PrintStyledContent(prompt_message)).unwrap();
        stdout.flush().unwrap();

        loop {
            if event::poll(std::time::Duration::from_secs(1)).unwrap() {
                match event::read().unwrap() {
                    Event::Key(event) => match event.code {
                        KeyCode::Char('y') | KeyCode::Char('Y') => {
                            diesel::update(env_vars.filter(env_key.eq(&key)))
                                .set(env_value.eq(&value))
                                .execute(&mut connection)
                                .expect("Error updating env var");
                            println!("Environment variable updated: {} = {}", key, value);
                            break;
                        }
                        KeyCode::Char('n') | KeyCode::Char('N') => {
                            println!("Update cancelled.");
                            break;
                        }
                        _ => (),
                    },
                    _ => (),
                }
            }
        }
    } else {
        let env_var = EnvVar {
            id: None,
            key: key.to_string(),
            value: value.to_string(),
        };

        diesel::insert_into(env_vars)
            .values(&env_var)
            .execute(&mut connection)
            .expect("Error saving new env var");

        println!("Environment variable set: {} = {}", key, value);
    }

    // Reload the environment variables from the database
    reload_env_command();

    // Call list_env_command to display the updated list of environment variables
    list_env_command();
}

// Reload the environment variables from the sqlite database
pub fn reload_env_command() {
    let mut stdout = stdout();
    let greeting = "Loaded environment variables...\n".green();
    stdout.execute(PrintStyledContent(greeting)).unwrap();
    stdout.flush().unwrap();

    // Load environment variables from .env file
    dotenv().ok();

    // Establish connection to the database
    let mut connection = establish_connection();

    // Load the environment variables from the database
    let results = env_vars::table
        .select(EnvVar::as_select())
        .load::<EnvVar>(&mut connection)
        .expect("Error loading env vars");

    // Set the environment variables
    for env_var in results {
        env::set_var(&env_var.key, &env_var.value);
    }
}

// Remove environment variables from that system the database, and after clean the table
pub fn clear_env_command() {
    let mut stdout = stdout();
    let greeting = "Clearing environment variables".green();
    stdout.execute(PrintStyledContent(greeting)).unwrap();
    stdout.flush().unwrap();

    // Establish connection to the database
    let mut connection = establish_connection();

    // Load the environment variables from the database
    let results = env_vars::table
        .select(EnvVar::as_select())
        .load::<EnvVar>(&mut connection)
        .expect("Error loading env vars");

    // Remove the environment variables from the system
    for env_var in results {
        env::remove_var(&env_var.key);
    }

    // Clear the table
    diesel::delete(env_vars::table)
        .execute(&mut connection)
        .expect("Error clearing env vars");

    println!("Environment variables cleared");
}