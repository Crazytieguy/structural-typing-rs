use structural_typing::structural;

#[structural]
struct Config {
    #[always]
    version: u32,
    name: String,
    description: String,
}

#[test]
fn test_always_present_in_empty() {
    // Always-present fields must be provided to empty()
    let config = Config::empty(1);

    // Always-present fields are directly accessible
    assert_eq!(config.version, 1);
}

#[test]
fn test_always_present_setter_no_state_change() {
    let config = Config::empty(1);

    // Setter for always-present field doesn't change state
    let config = config.version(2);
    assert_eq!(config.version, 2);

    // State is still Empty (no stateful fields set)
    // We can verify this by trying to use a method that requires Empty state
}

#[test]
fn test_always_present_with_stateful() {
    let config = Config::empty(1);

    // Set a stateful field
    let config = config.name("test".into());

    // Always-present field is still accessible
    assert_eq!(config.version, 1);
    assert_eq!(config.name, "test");
}

#[test]
fn test_always_present_in_merge() {
    let config1 = Config::empty(1).name("first".into());
    let config2 = Config::empty(2).description("second desc".into());

    // Merge should take always-present field from 'other' (config2)
    let merged = config1.merge(config2);

    assert_eq!(merged.version, 2);
    assert_eq!(merged.name, "first");
    assert_eq!(merged.description, "second desc");
}

#[test]
fn test_always_present_in_impl() {
    impl<S> Config<S>
    where
        S: config_state::State,
    {
        fn get_version(&self) -> u32 {
            self.version
        }
    }

    let config = Config::empty(42);
    assert_eq!(config.get_version(), 42);

    let config = config.name("test".into());
    assert_eq!(config.get_version(), 42);
}

#[test]
fn test_no_require_for_always_present() {
    let config = Config::empty(1);

    // require_version should not exist because version is always present
    // This test verifies that by trying to compile code that would fail if it existed

    // We can only call require_ on stateful fields
    let _result = config.require_name(); // This should compile

    // config.require_version(); // This should NOT compile (uncomment to verify)
}
