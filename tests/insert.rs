#![expect(missing_docs, reason = "Tests")]

use core::error::Error;

use similar_asserts::assert_eq;
use wayfind::{InsertError, RouterBuilder};

#[test]
fn insert_conflict_static() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/test", 1)?;

    let error = builder.insert("/test", 2).unwrap_err();
    assert_eq!(error, InsertError::Conflict {
        existing: "/test".to_owned(),
    });

    insta::assert_snapshot!(error, @"conflicts with `/test`");

    let router = builder.build();
    insta::assert_snapshot!(router, @"/test");

    Ok(())
}

#[test]
fn insert_conflict_dynamic() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<id>", 1)?;

    let error = builder.insert("/<id>", 2).unwrap_err();
    assert_eq!(error, InsertError::Conflict {
        existing: "/<id>".to_owned(),
    });

    insta::assert_snapshot!(error, @"conflicts with `/<id>`");

    let router = builder.build();
    insta::assert_snapshot!(router, @r"
    /
    ╰─ <id>
    ");

    Ok(())
}

#[test]
fn insert_conflict_dynamic_structural() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<id>", 1)?;

    let error = builder.insert("/<user>", 2).unwrap_err();
    assert_eq!(error, InsertError::Conflict {
        existing: "/<id>".to_owned(),
    });

    insta::assert_snapshot!(error, @"conflicts with `/<id>`");

    let router = builder.build();
    insta::assert_snapshot!(router, @r"
    /
    ╰─ <id>
    ");

    Ok(())
}

#[test]
fn insert_conflict_wildcard() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<*path>/edit", 1)?;

    let error = builder.insert("/<*path>/edit", 2).unwrap_err();
    assert_eq!(error, InsertError::Conflict {
        existing: "/<*path>/edit".to_owned(),
    });

    insta::assert_snapshot!(error, @"conflicts with `/<*path>/edit`");

    let router = builder.build();
    insta::assert_snapshot!(router, @r"
    /
    ╰─ <*path>
       ╰─ /edit
    ");

    Ok(())
}

#[test]
fn insert_conflict_wildcard_structural() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<*path>/edit", 1)?;

    let error = builder.insert("/<*slug>/edit", 2).unwrap_err();
    assert_eq!(error, InsertError::Conflict {
        existing: "/<*path>/edit".to_owned(),
    });

    insta::assert_snapshot!(error, @"conflicts with `/<*path>/edit`");

    let router = builder.build();
    insta::assert_snapshot!(router, @r"
    /
    ╰─ <*path>
       ╰─ /edit
    ");

    Ok(())
}

#[test]
fn insert_conflict_end_wildcard() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<*catch_all>", 1)?;

    let error = builder.insert("/<*catch_all>", 2).unwrap_err();
    assert_eq!(error, InsertError::Conflict {
        existing: "/<*catch_all>".to_owned(),
    });

    insta::assert_snapshot!(error, @"conflicts with `/<*catch_all>`");

    let router = builder.build();
    insta::assert_snapshot!(router, @r"
    /
    ╰─ <*catch_all>
    ");

    Ok(())
}

#[test]
fn insert_conflict_end_wildcard_structural() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<*catch_all>", 1)?;

    let error = builder.insert("/<*files>", 2).unwrap_err();
    assert_eq!(error, InsertError::Conflict {
        existing: "/<*catch_all>".to_owned(),
    });

    insta::assert_snapshot!(error, @"conflicts with `/<*catch_all>`");

    let router = builder.build();
    insta::assert_snapshot!(router, @r"
    /
    ╰─ <*catch_all>
    ");

    Ok(())
}

#[test]
fn insert_duplicate_parameter() {
    let mut builder = RouterBuilder::new();
    let error = builder.insert("/<*id>/users/<id>", 3).unwrap_err();
    assert_eq!(error, InsertError::DuplicateParameter {
        name: "id".to_owned(),
    });

    insta::assert_snapshot!(error, @"duplicate parameter name `id`");

    let router = builder.build();
    insta::assert_snapshot!(router, @"");
}
