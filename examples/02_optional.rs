//! Optional field state.

use structural_typing::structural;

#[structural]
struct Config {
    host: String,
    timeout: u64,
}

fn main() {
    // Optional fields - may or may not have a value
    let mut config = Config::empty()
        .host("localhost".to_owned())
        .timeout(Some(30));
    assert_eq!(config.timeout, Some(30));

    // Mutate Optional field directly
    config.timeout = None;
    assert_eq!(config.timeout, None);
}
