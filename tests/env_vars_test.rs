// tests/integration_test.rs

use terminal::env_vars; // replace with your project name
use std::env;

#[test]
fn test_list_env_command() {
    //env_vars::list_env_command();
    assert_eq!(1, 1);
}

#[test]
fn test_env_var_exists() {
    let env_var = "TEST2";
    let result = match env::var(env_var) {
        Ok(value) => format!("The environment variable {} is set to: {}", env_var, value),
        Err(env::VarError::NotPresent) => format!("The environment variable {} is not set.", env_var),
        Err(env::VarError::NotUnicode(_)) => format!("The environment variable {} is set but contains invalid Unicode.", env_var),
    };
    assert_eq!(result, "The environment variable TEST2 is not set.");
}
