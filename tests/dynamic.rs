#![expect(missing_docs, clippy::panic_in_result_fn, reason = "Tests")]

use core::error::Error;

use similar_asserts::assert_eq;
use wayfind::RouterBuilder;

#[test]
fn dynamic_simple() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<id>", 1)?;

    let router = builder.build();
    insta::assert_snapshot!(router, @r"
    /
    ╰─ <id>
    ");

    let search = router.search("/123").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<id>");
    assert_eq!(search.parameters(), &[("id", "123")]);

    let search = router.search("/");
    assert!(search.is_none());

    Ok(())
}

#[test]
fn dynamic_multiple() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<year>", 1)?;
    builder.insert("/<year>/<month>", 2)?;
    builder.insert("/<year>/<month>/<day>", 3)?;

    let router = builder.build();
    insta::assert_snapshot!(router, @r"
    /
    ╰─ <year>
       ╰─ /
          ╰─ <month>
             ╰─ /
                ╰─ <day>
    ");

    let search = router.search("/2024").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<year>");
    assert_eq!(search.parameters(), &[("year", "2024")]);

    let search = router.search("/2024/12").unwrap();
    assert_eq!(search.data(), &2);
    assert_eq!(search.template(), "/<year>/<month>");
    assert_eq!(search.parameters(), &[("year", "2024"), ("month", "12")]);

    let search = router.search("/2024/12/01").unwrap();
    assert_eq!(search.data(), &3);
    assert_eq!(search.template(), "/<year>/<month>/<day>");
    assert_eq!(search.parameters(), &[
        ("year", "2024"),
        ("month", "12"),
        ("day", "01")
    ]);

    Ok(())
}

#[test]
fn dynamic_priority() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/robots.txt", 1)?;
    builder.insert("/<name>.txt", 2)?;
    builder.insert("/<name>", 3)?;

    let router = builder.build();
    insta::assert_snapshot!(router, @r"
    /
    ├─ robots.txt
    ╰─ <name>
       ╰─ .txt
    ");

    let search = router.search("/robots.txt").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/robots.txt");
    assert_eq!(search.parameters(), &[]);

    let search = router.search("/config.txt").unwrap();
    assert_eq!(search.data(), &2);
    assert_eq!(search.template(), "/<name>.txt");
    assert_eq!(search.parameters(), &[("name", "config")]);

    let search = router.search("/config.pdf").unwrap();
    assert_eq!(search.data(), &3);
    assert_eq!(search.template(), "/<name>");
    assert_eq!(search.parameters(), &[("name", "config.pdf")]);

    let search = router.search("/.txt").unwrap();
    assert_eq!(search.data(), &3);
    assert_eq!(search.template(), "/<name>");
    assert_eq!(search.parameters(), &[("name", ".txt")]);

    Ok(())
}

#[test]
fn dynamic_suffix() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<aaa>", 1)?;
    builder.insert("/<zzz>.txt", 2)?;

    let router = builder.build();

    let search = router.search("/hello.txt").unwrap();
    assert_eq!(search.data(), &2);
    assert_eq!(search.template(), "/<zzz>.txt");
    assert_eq!(search.parameters(), &[("zzz", "hello")]);

    let search = router.search("/hello.pdf").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<aaa>");
    assert_eq!(search.parameters(), &[("aaa", "hello.pdf")]);

    Ok(())
}

#[test]
fn dynamic_suffix_reversed() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<zzz>", 1)?;
    builder.insert("/<aaa>.txt", 2)?;

    let router = builder.build();

    let search = router.search("/hello.txt").unwrap();
    assert_eq!(search.data(), &2);
    assert_eq!(search.template(), "/<aaa>.txt");
    assert_eq!(search.parameters(), &[("aaa", "hello")]);

    let search = router.search("/hello.pdf").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<zzz>");
    assert_eq!(search.parameters(), &[("zzz", "hello.pdf")]);

    Ok(())
}

#[test]
fn dynamic_mixed() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<a>/one", 1)?;
    builder.insert("/<b>/two", 2)?;
    builder.insert("/<c>.json", 3)?;

    let router = builder.build();

    let search = router.search("/hello/one").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<a>/one");
    assert_eq!(search.parameters(), &[("a", "hello")]);

    let search = router.search("/hello/two").unwrap();
    assert_eq!(search.data(), &2);
    assert_eq!(search.template(), "/<b>/two");
    assert_eq!(search.parameters(), &[("b", "hello")]);

    let search = router.search("/hello.json").unwrap();
    assert_eq!(search.data(), &3);
    assert_eq!(search.template(), "/<c>.json");
    assert_eq!(search.parameters(), &[("c", "hello")]);

    Ok(())
}

#[test]
fn dynamic_sibling() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<a>.txt", 1)?;
    builder.insert("/<a>", 2)?;
    builder.insert("/<b>.json", 3)?;

    let router = builder.build();

    let search = router.search("/hello.txt").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<a>.txt");
    assert_eq!(search.parameters(), &[("a", "hello")]);

    let search = router.search("/hello.json").unwrap();
    assert_eq!(search.data(), &3);
    assert_eq!(search.template(), "/<b>.json");
    assert_eq!(search.parameters(), &[("b", "hello")]);

    let search = router.search("/hello.pdf").unwrap();
    assert_eq!(search.data(), &2);
    assert_eq!(search.template(), "/<a>");
    assert_eq!(search.parameters(), &[("a", "hello.pdf")]);

    let search = router.search("/a.txt").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<a>.txt");
    assert_eq!(search.parameters(), &[("a", "a")]);

    Ok(())
}

#[test]
fn dynamic_inline_params() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<id>.<format>", 1)?;

    let router = builder.build();
    insta::assert_snapshot!(router, @r"
    /
    ╰─ <id>
       ╰─ .
          ╰─ <format>
    ");

    let search = router.search("/archive.tar.gz").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<id>.<format>");
    assert_eq!(search.parameters(), &[
        ("id", "archive.tar"),
        ("format", "gz")
    ]);

    let search = router.search("/report.pdf").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<id>.<format>");
    assert_eq!(search.parameters(), &[("id", "report"), ("format", "pdf")]);

    let search = router.search("/nodots");
    assert!(search.is_none());

    Ok(())
}

#[test]
fn dynamic_triple_params() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<a>.<b>.<c>", 1)?;

    let router = builder.build();
    insta::assert_snapshot!(router, @r"
    /
    ╰─ <a>
       ╰─ .
          ╰─ <b>
             ╰─ .
                ╰─ <c>
    ");

    let search = router.search("/x.y.z").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<a>.<b>.<c>");
    assert_eq!(search.parameters(), &[("a", "x"), ("b", "y"), ("c", "z")]);

    let search = router.search("/a.b.c.d").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<a>.<b>.<c>");
    assert_eq!(search.parameters(), &[("a", "a.b"), ("b", "c"), ("c", "d")]);

    let search = router.search("/one.two.");
    assert!(search.is_none());

    Ok(())
}

#[test]
fn dynamic_multi_separator() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<from>...<to>", 1)?;

    let router = builder.build();
    insta::assert_snapshot!(router, @r"
    /
    ╰─ <from>
       ╰─ ...
          ╰─ <to>
    ");

    let search = router.search("/10...100").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<from>...<to>");
    assert_eq!(search.parameters(), &[("from", "10"), ("to", "100")]);

    let search = router.search("/abc...xyz").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<from>...<to>");
    assert_eq!(search.parameters(), &[("from", "abc"), ("to", "xyz")]);

    let search = router.search("/nodots");
    assert!(search.is_none());

    Ok(())
}

#[test]
fn dynamic_inline_coexistence() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<id>.<format>", 1)?;
    builder.insert("/<name>.txt", 2)?;

    let router = builder.build();

    let search = router.search("/report.txt").unwrap();
    assert_eq!(search.data(), &2);
    assert_eq!(search.template(), "/<name>.txt");
    assert_eq!(search.parameters(), &[("name", "report")]);

    let search = router.search("/report.pdf").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<id>.<format>");
    assert_eq!(search.parameters(), &[("id", "report"), ("format", "pdf")]);

    let search = router.search("/archive.tar.gz").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<id>.<format>");
    assert_eq!(search.parameters(), &[
        ("id", "archive.tar"),
        ("format", "gz")
    ]);

    Ok(())
}

#[test]
fn dynamic_repeated_suffix() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<name>.txt", 1)?;
    builder.insert("/<name>", 2)?;

    let router = builder.build();

    let search = router.search("/a.txt.txt.txt.txt").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<name>.txt");
    assert_eq!(search.parameters(), &[("name", "a.txt.txt.txt")]);

    let search = router.search("/.txt").unwrap();
    assert_eq!(search.data(), &2);
    assert_eq!(search.template(), "/<name>");
    assert_eq!(search.parameters(), &[("name", ".txt")]);

    Ok(())
}

#[test]
fn dynamic_overlapping_suffix() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<from>...<to>", 1)?;
    builder.insert("/<from>", 2)?;

    let router = builder.build();
    insta::assert_snapshot!(router, @r"
    /
    ╰─ <from>
       ╰─ ...
          ╰─ <to>
    ");

    let search = router.search("/10...100").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<from>...<to>");
    assert_eq!(search.parameters(), &[("from", "10"), ("to", "100")]);

    let search = router.search("/10").unwrap();
    assert_eq!(search.data(), &2);
    assert_eq!(search.template(), "/<from>");
    assert_eq!(search.parameters(), &[("from", "10")]);

    let search = router.search("/1....").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<from>...<to>");
    assert_eq!(search.parameters(), &[("from", "1"), ("to", ".")]);

    Ok(())
}

#[test]
fn dynamic_multibyte() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<name>.txt", 1)?;

    let router = builder.build();

    let search = router.search("/café.txt").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<name>.txt");
    assert_eq!(search.parameters(), &[("name", "café")]);

    let search = router.search("/日本語.txt").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<name>.txt");
    assert_eq!(search.parameters(), &[("name", "日本語")]);

    Ok(())
}

#[test]
fn dynamic_segment_slash() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<a>/<b>", 1)?;

    let router = builder.build();

    let search = router.search("//foo");
    assert!(search.is_none());

    Ok(())
}

#[test]
fn dynamic_segment_tails() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<name>/edit", 1)?;

    let router = builder.build();

    let search = router.search("/foo/view");
    assert!(search.is_none());

    Ok(())
}

#[test]
fn dynamic_inline_tails() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<a>.txt", 1)?;
    builder.insert("/<b>/edit", 2)?;

    let router = builder.build();

    let search = router.search("/foo/view");
    assert!(search.is_none());

    Ok(())
}

#[test]
fn dynamic_segment_revisit() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<a>x<b>x<c>/-/<d>/end/<e>", 1)?;

    let router = builder.build();

    let search = router.search("/xxxxxx/-/yyyyyy/end/");
    assert!(search.is_none());

    Ok(())
}

#[test]
fn dynamic_overlapping_boundaries() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<a>ab", 1)?;
    builder.insert("/<a>a<b>", 2)?;

    let router = builder.build();

    let search = router.search("/xab").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<a>ab");
    assert_eq!(search.parameters(), &[("a", "x")]);

    let search = router.search("/xab/Q");
    assert!(search.is_none());

    Ok(())
}
