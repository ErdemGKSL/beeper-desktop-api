//! Common test utilities and setup

use std::env;

pub fn get_test_token() -> Option<String> {
    env::var("BEEPER_TEST_TOKEN").ok()
}

pub fn get_test_base_url() -> String {
    env::var("BEEPER_TEST_URL")
        .unwrap_or_else(|_| "http://localhost:23373".to_string())
}

pub fn should_run_integration_tests() -> bool {
    get_test_token().is_some()
}
