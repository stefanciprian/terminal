//#[macro_use]
//extern crate diesel;

use crossterm::style::{PrintStyledContent, Stylize};
use crossterm::ExecutableCommand; // Import the necessary trait
use diesel::prelude::*;
use diesel::sqlite::{Sqlite, SqliteConnection};
use dotenvy::dotenv;
use std::env;
use std::io::{self, stdout, Write};

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

// This function is moved to `commands.rs`
pub fn env_vars_command() {
    let mut stdout = stdout();
    let greeting = "Hello from Rust! You've executed the 'env_vars' command.".green();
    stdout.execute(PrintStyledContent(greeting)).unwrap(); // Now `execute` can be used here
    stdout.flush().unwrap();

    let mut connection = establish_connection();

    // Load environment variables from .env file
    dotenv().ok();

    // Insert environment variables into the database
    for (key, value) in env::vars() {
        let env_var = EnvVar {
            id: None,
            key: key.clone(),
            value: value.clone(),
        };

        diesel::insert_into(env_vars::table)
            .values(&env_var)
            .execute(&mut connection) // Pass mutable reference
            .expect("Error saving new env var");
    }

    // Query environment variables
    let results = env_vars::table
        .select(EnvVar::as_select())  // Use the Selectable trait to match struct fields
        .load::<EnvVar>(&mut connection) // Pass mutable reference
        .expect("Error loading env vars");

    println!("Displaying {} env vars", results.len());
    for env_var in results {
        println!("{}: {}", env_var.key, env_var.value);
    }
}
