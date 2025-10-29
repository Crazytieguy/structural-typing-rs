//! Optional and Absent field states.

use structural_typing::structural;

#[structural]
struct Config {
    host: String,
    port: u16,
    timeout: u64,
}

fn main() {
    // Optional fields - may or may not have a value
    let config = Config::empty()
        .host("localhost".into())
        .maybe_timeout(Some(30));
    assert_eq!(config.timeout, Some(30));

    let config = config.maybe_timeout(None);
    assert_eq!(config.timeout, None);

    // Absent fields - cannot be accessed directly
    let config = Config::empty().host("localhost".into()).unset_port();

    // Use getter to check presence
    assert!(config.get_port().is_none());
    assert_eq!(config.get_host(), Some(&"localhost".to_string()));
}
