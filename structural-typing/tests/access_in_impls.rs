use structural_typing::{structural, Access, Present};

#[structural]
struct Config {
    host: String,
    port: u16,
    timeout: u64,
    debug: bool,
}

impl<S> Config<S>
where
    S: config_state::State<Host = Present, Port = Present>,
{
    fn connection_string(&self) -> String {
        let timeout_str = match self.timeout.get() {
            Some(t) => format!(" (timeout: {}s)", t),
            None => String::new(),
        };

        let debug_str = match self.debug.get() {
            Some(true) => " [DEBUG]",
            Some(false) => "",
            None => "",
        };

        format!("{}:{}{}{}", self.host, self.port, timeout_str, debug_str)
    }
}

#[test]
fn test_access_methods_work_in_generic_impl() {
    let basic = Config::empty()
        .host("localhost".into())
        .port(8080);

    assert_eq!(basic.connection_string(), "localhost:8080");

    let with_timeout = basic.timeout(30);
    assert_eq!(with_timeout.connection_string(), "localhost:8080 (timeout: 30s)");

    let with_debug = with_timeout.debug(true);
    assert_eq!(with_debug.connection_string(), "localhost:8080 (timeout: 30s) [DEBUG]");
}

#[test]
fn test_get_works_on_all_presence_states() {
    let empty = Config::empty().host("example.com".into()).port(443);

    assert_eq!(empty.host.get(), Some(&"example.com".to_string()));
    assert_eq!(empty.port.get(), Some(&443));
    assert_eq!(empty.timeout.get(), Option::<&u64>::None);
    assert_eq!(empty.debug.get(), Option::<&bool>::None);

    let full = empty.timeout(60).debug(false);

    assert_eq!(full.timeout.get(), Some(&60));
    assert_eq!(full.debug.get(), Some(&false));
}
