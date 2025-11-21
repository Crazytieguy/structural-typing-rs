//! Optional field state.

use structural_typing::structural;

#[structural]
struct Config {
    host: String,
    timeout: u64,
}

fn main() {
    let mut config = config::empty()
        .host("localhost".to_owned())
        .timeout(Some(30));
    assert_eq!(config.timeout, Some(30));

    config.timeout = None;
    assert_eq!(config.timeout, None);
}
