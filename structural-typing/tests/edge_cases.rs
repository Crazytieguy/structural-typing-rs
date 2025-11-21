use structural_typing::{select, structural};

#[structural]
#[derive(Debug)]
struct RawIdConfig {
    r#type: String,
    r#match: bool,
    normal: u32,
}

#[structural]
struct SingleField {
    data: String,
}

#[test]
fn raw_identifiers() {
    let cfg = RawIdConfig::empty()
        .r#type("test".to_owned())
        .r#match(true)
        .normal(42);

    assert_eq!(cfg.r#type, "test");
    assert!(cfg.r#match);
    assert_eq!(cfg.normal, 42);
}

#[test]
fn raw_identifiers_extract() {
    let cfg = RawIdConfig::empty()
        .r#type("test".to_owned())
        .r#match(true)
        .normal(42);

    let (extracted, remainder) = cfg.extract::<select!(raw_id_config: r#type, r#match)>();
    assert_eq!(extracted.r#type, "test");
    assert!(extracted.r#match);
    assert_eq!(remainder.normal, 42);
}

#[test]
fn raw_identifiers_try_extract() {
    let cfg = RawIdConfig::empty()
        .r#type(Some("optional".to_owned()))
        .r#match(Some(false))
        .normal(99);

    let result = cfg.try_extract::<select!(raw_id_config: r#type, r#match)>();
    assert!(result.is_ok());
    let (extracted, remainder) = result.unwrap();
    assert_eq!(extracted.r#type, "optional");
    assert!(!extracted.r#match);
    assert_eq!(remainder.normal, 99);

    let partial = RawIdConfig::empty()
        .r#type(Some("value".to_owned()))
        .r#match(None)
        .normal(77);

    let result = partial.try_extract::<select!(raw_id_config: r#type, r#match)>();
    assert!(result.is_err());
}

#[test]
fn single_field_struct() {
    type DataPresent = select!(single_field: data);
    let val: SingleField<DataPresent> = SingleField::empty().data("test".to_owned());
    assert_eq!(val.data, "test");

    type DataOptional = select!(single_field: data?);
    let val2: SingleField<DataOptional> = SingleField::empty().data(Some("test".to_owned()));
    assert_eq!(val2.data, Some("test".to_owned()));

    let val3: SingleField<DataOptional> = SingleField::empty().data(None);
    assert_eq!(val3.data, None);
}
