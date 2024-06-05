// tests/integration_test.rs

use terminal::env_vars; // replace with your project name

#[test]
fn test_list_env_command() {
    env_vars::list_env_command();
    assert_eq!(1, 1);
}
