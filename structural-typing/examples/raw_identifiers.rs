use structural_typing::structural;

#[structural]
#[derive(Clone, Debug)]
struct Config {
    r#type: String,
    r#match: u32,
    normal_field: bool,
}

fn main() {
    // Test basic construction with raw identifiers
    let config = Config::empty()
        .r#type("production".into())
        .r#match(42)
        .normal_field(true);

    assert_eq!(config.r#type, "production");
    assert_eq!(config.r#match, 42);
    assert!(config.normal_field);

    // Test select! with raw identifiers
    let selected = config.project::<config::select!(r#type, normal_field)>();
    assert_eq!(selected.r#type, "production");
    assert!(selected.normal_field);

    // Test modify! with raw identifiers
    type TypeOnly = config::modify!(config::AllAbsent, +r#type);
    let type_only: Config<TypeOnly> = Config::empty().r#type("test".into());
    assert_eq!(type_only.r#type, "test");

    println!("✓ Raw identifier tests passed!");
}
