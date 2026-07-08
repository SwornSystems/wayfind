#![expect(missing_docs, clippy::panic_in_result_fn, reason = "Tests")]

use core::error::Error;

use similar_asserts::assert_eq;
use wayfind::RouterBuilder;

#[test]
fn wildcard_simple() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<*path>/delete", 1)?;

    let router = builder.build();
    insta::assert_snapshot!(router, @r"
    /
    ╰─ <*path>
       ╰─ /delete
    ");

    let search = router.search("/docs/delete").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<*path>/delete");
    assert_eq!(search.parameters(), &[("path", "docs")]);

    let search = router.search("/nested/docs/folder/delete").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<*path>/delete");
    assert_eq!(search.parameters(), &[("path", "nested/docs/folder")]);

    let search = router.search("/delete");
    assert!(search.is_none());

    Ok(())
}

#[test]
fn wildcard_multiple() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<*prefix>/static/<*suffix>/file", 1)?;

    let router = builder.build();
    insta::assert_snapshot!(router, @r"
    /
    ╰─ <*prefix>
       ╰─ /static/
          ╰─ <*suffix>
             ╰─ /file
    ");

    let search = router.search("/a/static/b/file").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<*prefix>/static/<*suffix>/file");
    assert_eq!(search.parameters(), &[("prefix", "a"), ("suffix", "b")]);

    let search = router.search("/a/b/c/static/d/e/f/file").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<*prefix>/static/<*suffix>/file");
    assert_eq!(search.parameters(), &[
        ("prefix", "a/b/c"),
        ("suffix", "d/e/f")
    ]);

    Ok(())
}

#[test]
fn wildcard_inline() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<*path>.html", 1)?;

    let router = builder.build();
    insta::assert_snapshot!(router, @r"
    /
    ╰─ <*path>
       ╰─ .html
    ");

    let search = router.search("/page.html").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<*path>.html");
    assert_eq!(search.parameters(), &[("path", "page")]);

    let search = router.search("/nested/page.html").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<*path>.html");
    assert_eq!(search.parameters(), &[("path", "nested/page")]);

    let search = router.search("/.html");
    assert!(search.is_none());

    Ok(())
}

#[test]
fn wildcard_empty() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<*path>/end", 1)?;

    let router = builder.build();
    insta::assert_snapshot!(router, @r"
    /
    ╰─ <*path>
       ╰─ /end
    ");

    let search = router.search("/start/middle/end").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<*path>/end");
    assert_eq!(search.parameters(), &[("path", "start/middle")]);

    let search = router.search("/start//middle///end").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<*path>/end");
    assert_eq!(search.parameters(), &[("path", "start//middle//")]);

    Ok(())
}

#[test]
fn wildcard_priority() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/static/path", 1)?;
    builder.insert("/static/<*rest>", 2)?;
    builder.insert("/<*path>/static", 3)?;
    builder.insert("/prefix.<*suffix>", 4)?;
    builder.insert("/<*prefix>.suffix", 5)?;

    let router = builder.build();
    insta::assert_snapshot!(router, @"
    /
    ├─ prefix.
    │  ╰─ <*suffix>
    ├─ static/
    │  ├─ path
    │  ╰─ <*rest>
    ├─ <*path>
    │  ╰─ /static
    ╰─ <*prefix>
       ╰─ .suffix
    ");

    let search = router.search("/static/path").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/static/path");
    assert_eq!(search.parameters(), &[]);

    let search = router.search("/static/some/nested/path").unwrap();
    assert_eq!(search.data(), &2);
    assert_eq!(search.template(), "/static/<*rest>");
    assert_eq!(search.parameters(), &[("rest", "some/nested/path")]);

    let search = router.search("/some/nested/path/static").unwrap();
    assert_eq!(search.data(), &3);
    assert_eq!(search.template(), "/<*path>/static");
    assert_eq!(search.parameters(), &[("path", "some/nested/path")]);

    let search = router.search("/prefix.some/nested/path").unwrap();
    assert_eq!(search.data(), &4);
    assert_eq!(search.template(), "/prefix.<*suffix>");
    assert_eq!(search.parameters(), &[("suffix", "some/nested/path")]);

    let search = router.search("/some/nested/path.suffix").unwrap();
    assert_eq!(search.data(), &5);
    assert_eq!(search.template(), "/<*prefix>.suffix");
    assert_eq!(search.parameters(), &[("prefix", "some/nested/path")]);

    Ok(())
}

#[test]
fn wildcard_greedy() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<*path>/edit/<*rest>", 1)?;
    builder.insert("/<*path>/delete", 2)?;

    let router = builder.build();
    insta::assert_snapshot!(router, @"
    /
    ╰─ <*path>
       ╰─ /
          ├─ delete
          ╰─ edit/
             ╰─ <*rest>
    ");

    let search = router.search("/documents/delete").unwrap();
    assert_eq!(search.data(), &2);
    assert_eq!(search.template(), "/<*path>/delete");
    assert_eq!(search.parameters(), &[("path", "documents")]);

    let search = router.search("/documents/edit/summary").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<*path>/edit/<*rest>");
    assert_eq!(search.parameters(), &[
        ("path", "documents"),
        ("rest", "summary")
    ]);

    let search = router.search("/docs/edit/readme/delete").unwrap();
    assert_eq!(search.data(), &2);
    assert_eq!(search.template(), "/<*path>/delete");
    assert_eq!(search.parameters(), &[("path", "docs/edit/readme")]);

    Ok(())
}

#[test]
fn wildcard_dynamic() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/api/<version>/<*rest>", 1)?;
    builder.insert("/api/<*path>/help", 2)?;

    let router = builder.build();
    insta::assert_snapshot!(router, @r"
    /api/
    ├─ <version>
    │  ╰─ /
    │     ╰─ <*rest>
    ╰─ <*path>
       ╰─ /help
    ");

    // NOTE: Should arguably match `/<*path>/help`.
    let search = router.search("/api/docs/reference/help").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/api/<version>/<*rest>");
    assert_eq!(search.parameters(), &[
        ("version", "docs"),
        ("rest", "reference/help")
    ]);

    Ok(())
}

#[test]
fn wildcard_suffix() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<*aaa>", 1)?;
    builder.insert("/<*zzz>.txt", 2)?;

    let router = builder.build();

    let search = router.search("/hello.txt").unwrap();
    assert_eq!(search.data(), &2);
    assert_eq!(search.template(), "/<*zzz>.txt");
    assert_eq!(search.parameters(), &[("zzz", "hello")]);

    let search = router.search("/hello.pdf").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<*aaa>");
    assert_eq!(search.parameters(), &[("aaa", "hello.pdf")]);

    Ok(())
}

#[test]
fn wildcard_fallthrough() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<username>", 1)?;
    builder.insert("/<*namespace_id>/<project_id>/-/issues", 2)?;

    let router = builder.build();

    let search = router.search("/johndoe").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<username>");
    assert_eq!(search.parameters(), &[("username", "johndoe")]);

    let search = router.search("/gitlab-org/gitlab/-/issues").unwrap();
    assert_eq!(search.data(), &2);
    assert_eq!(search.template(), "/<*namespace_id>/<project_id>/-/issues");
    assert_eq!(search.parameters(), &[
        ("namespace_id", "gitlab-org"),
        ("project_id", "gitlab")
    ]);

    Ok(())
}

#[test]
fn wildcard_namespace() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<*namespace_id>/<project_id>/-/merge_requests", 1)?;

    let router = builder.build();

    let search = router
        .search("/gitlab-org/frontend/gitlab-ui/-/merge_requests")
        .unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(
        search.template(),
        "/<*namespace_id>/<project_id>/-/merge_requests"
    );
    assert_eq!(search.parameters(), &[
        ("namespace_id", "gitlab-org/frontend"),
        ("project_id", "gitlab-ui")
    ],);

    Ok(())
}

#[test]
fn wildcard_catchall() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<username>", 1)?;
    builder.insert("/<username>/<*rest>", 2)?;
    builder.insert("/<*namespace_id>/<project_id>/-/issues", 3)?;

    let router = builder.build();
    insta::assert_snapshot!(router, @r"
    /
    ├─ <username>
    │  ╰─ /
    │     ╰─ <*rest>
    ╰─ <*namespace_id>
       ╰─ /
          ╰─ <project_id>
             ╰─ /-/issues
    ");

    // NOTE: Should arguably match `/<*namespace_id>/<project_id>/-/issues`.
    let search = router.search("/gitlab-org/gitlab/-/issues").unwrap();
    assert_eq!(search.data(), &2);
    assert_eq!(search.template(), "/<username>/<*rest>");
    assert_eq!(search.parameters(), &[
        ("username", "gitlab-org"),
        ("rest", "gitlab/-/issues")
    ]);

    Ok(())
}

#[test]
fn wildcard_miss_fallback() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/pipelines/<id>", 1)?;
    builder.insert("/pipelines/<id>/security", 2)?;
    builder.insert("/pipelines/<*ref>/latest", 3)?;

    let router = builder.build();

    let search = router.search("/pipelines/main/latest").unwrap();
    assert_eq!(search.data(), &3);
    assert_eq!(search.template(), "/pipelines/<*ref>/latest");
    assert_eq!(search.parameters(), &[("ref", "main")]);

    Ok(())
}

#[test]
fn wildcard_multi_segment() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/pipelines/<*ref>/latest", 1)?;

    let router = builder.build();

    let search = router.search("/pipelines/feature/auth/latest").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/pipelines/<*ref>/latest");
    assert_eq!(search.parameters(), &[("ref", "feature/auth")]);

    Ok(())
}

#[test]
fn wildcard_nested() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<*a>/<b>/-/issues", 1)?;

    let router = builder.build();

    let search = router.search("/a/b/c/my-project/-/issues").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<*a>/<b>/-/issues");
    assert_eq!(search.parameters(), &[("a", "a/b/c"), ("b", "my-project")]);

    Ok(())
}

#[test]
fn wildcard_branches() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<*a>/<b>/-/wikis/<c>", 1)?;
    builder.insert("/<*a>/-/settings", 2)?;

    let router = builder.build();

    let search = router.search("/gitlab-org/gitlab/-/wikis/home").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<*a>/<b>/-/wikis/<c>");
    assert_eq!(search.parameters(), &[
        ("a", "gitlab-org"),
        ("b", "gitlab"),
        ("c", "home")
    ]);

    let search = router.search("/gitlab-org/-/settings").unwrap();
    assert_eq!(search.data(), &2);
    assert_eq!(search.template(), "/<*a>/-/settings");
    assert_eq!(search.parameters(), &[("a", "gitlab-org")]);

    Ok(())
}

#[test]
fn wildcard_ambiguous() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<*a>/<b>/-/issues", 1)?;

    let router = builder.build();

    let search = router.search("/a/-/issues/real-project/-/issues").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<*a>/<b>/-/issues");
    assert_eq!(search.parameters(), &[
        ("a", "a/-/issues"),
        ("b", "real-project")
    ]);

    Ok(())
}

#[test]
fn wildcard_chain() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<*a>/middle/<*b>/end", 1)?;

    let router = builder.build();

    let search = router.search("/x/y/middle/w/v/end").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<*a>/middle/<*b>/end");
    assert_eq!(search.parameters(), &[("a", "x/y"), ("b", "w/v")]);

    Ok(())
}

#[test]
fn wildcard_multibyte() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<*path>.html", 1)?;

    let router = builder.build();

    let search = router.search("/docs/café.html").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<*path>.html");
    assert_eq!(search.parameters(), &[("path", "docs/café")]);

    let search = router.search("/日本語/ページ.html").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<*path>.html");
    assert_eq!(search.parameters(), &[("path", "日本語/ページ")]);

    Ok(())
}

#[test]
fn wildcard_repeated_anchor() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<*x>/-/<*y>/end", 1)?;

    let router = builder.build();

    let search = router.search("/a/-/b/-/c/-/d/-/end").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<*x>/-/<*y>/end");
    assert_eq!(search.parameters(), &[("x", "a/-/b/-/c"), ("y", "d/-")]);

    let search = router.search("/a/-/b/-/c/-/d/-/miss");
    assert!(search.is_none());

    Ok(())
}

#[test]
fn wildcard_endings() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<*a>/x", 1)?;
    builder.insert("/<*a>/y", 2)?;
    builder.insert("/<*a>/z", 3)?;

    let router = builder.build();
    insta::assert_snapshot!(router, @r"
    /
    ╰─ <*a>
       ╰─ /
          ├─ x
          ├─ y
          ╰─ z
    ");

    let search = router.search("/one/x").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<*a>/x");
    assert_eq!(search.parameters(), &[("a", "one")]);

    let search = router.search("/one/two/three/y").unwrap();
    assert_eq!(search.data(), &2);
    assert_eq!(search.template(), "/<*a>/y");
    assert_eq!(search.parameters(), &[("a", "one/two/three")]);

    let search = router.search("/one/z").unwrap();
    assert_eq!(search.data(), &3);
    assert_eq!(search.template(), "/<*a>/z");
    assert_eq!(search.parameters(), &[("a", "one")]);

    let search = router.search("/one/miss");
    assert!(search.is_none());

    Ok(())
}

#[test]
fn wildcard_anchored() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<*a>/-/<*b>/x", 1)?;
    builder.insert("/<*a>/-/<*b>/y", 2)?;
    builder.insert("/<*a>/-/<*b>/z", 3)?;

    let router = builder.build();

    let search = router.search("/one/two/-/three/x").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<*a>/-/<*b>/x");
    assert_eq!(search.parameters(), &[("a", "one/two"), ("b", "three")]);

    let search = router.search("/one/-/two/y").unwrap();
    assert_eq!(search.data(), &2);
    assert_eq!(search.template(), "/<*a>/-/<*b>/y");
    assert_eq!(search.parameters(), &[("a", "one"), ("b", "two")]);

    let search = router.search("/one/-/two/miss");
    assert!(search.is_none());

    let search = router.search("/-/x/-/x/-/x/miss");
    assert!(search.is_none());

    Ok(())
}

#[test]
fn wildcard_repeated_suffix() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<*path>.txt", 1)?;
    builder.insert("/<*path>", 2)?;

    let router = builder.build();

    let search = router.search("/a.txt.txt.txt.txt").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<*path>.txt");
    assert_eq!(search.parameters(), &[("path", "a.txt.txt.txt")]);

    let search = router.search("/.txt").unwrap();
    assert_eq!(search.data(), &2);
    assert_eq!(search.template(), "/<*path>");
    assert_eq!(search.parameters(), &[("path", ".txt")]);

    Ok(())
}

#[test]
fn wildcard_segment_empty() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<*a>/<b>", 1)?;

    let router = builder.build();

    let search = router.search("//foo");
    assert!(search.is_none());

    Ok(())
}

#[test]
fn wildcard_inline_empty() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<*path>.x/<*rest>", 1)?;

    let router = builder.build();

    let search = router.search("/.x/y");
    assert!(search.is_none());

    Ok(())
}

#[test]
fn wildcard_backtrack() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<*a>/-/<*b>/x", 1)?;

    let router = builder.build();

    let search = router.search("/x/-/y/-/x").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<*a>/-/<*b>/x");
    assert_eq!(search.parameters(), &[("a", "x"), ("b", "y/-")]);

    Ok(())
}

#[test]
fn wildcard_nested_backtrack() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<*a>/x/<*b>/y/<c>", 1)?;

    let router = builder.build();

    let search = router.search("/x/y/x/y/x/y/q").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<*a>/x/<*b>/y/<c>");
    assert_eq!(search.parameters(), &[
        ("a", "x/y"),
        ("b", "y/x"),
        ("c", "q")
    ]);

    Ok(())
}

#[test]
fn wildcard_revisit() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<*a>/x/<*b>/y/<c>", 1)?;

    let router = builder.build();

    let search = router.search("/x/n/x/q/x/m/x/y/");
    assert!(search.is_none());

    Ok(())
}

#[test]
fn wildcard_inline_backtrack() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<*a>.x/<b>", 1)?;

    let router = builder.build();

    let search = router.search("/b.x/c.x/d").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<*a>.x/<b>");
    assert_eq!(search.parameters(), &[("a", "b.x/c"), ("b", "d")]);

    let search = router.search("/b.x/c.x/");
    assert!(search.is_none());

    Ok(())
}

#[test]
fn wildcard_sibling() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<*a>/x", 1)?;
    builder.insert("/<*b>/forever", 2)?;

    let router = builder.build();

    let search = router.search("/q/x").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<*a>/x");
    assert_eq!(search.parameters(), &[("a", "q")]);

    let search = router.search("/q/forever").unwrap();
    assert_eq!(search.data(), &2);
    assert_eq!(search.template(), "/<*b>/forever");
    assert_eq!(search.parameters(), &[("b", "q")]);

    Ok(())
}

#[test]
fn wildcard_overlapping() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<*a>ab", 1)?;
    builder.insert("/<*a>a<b>", 2)?;

    let router = builder.build();

    let search = router.search("/xab").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<*a>ab");
    assert_eq!(search.parameters(), &[("a", "x")]);

    let search = router.search("/xab/Q");
    assert!(search.is_none());

    Ok(())
}

#[test]
fn wildcard_inline_sibling() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<*path>/<file>", 1)?;
    builder.insert("/<*path>.zip", 2)?;
    builder.insert("/<*path>/raw", 3)?;

    let router = builder.build();

    let search = router.search("/docs/raw/page").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<*path>/<file>");
    assert_eq!(search.parameters(), &[
        ("path", "docs/raw"),
        ("file", "page")
    ]);

    let search = router.search("/docs/raw").unwrap();
    assert_eq!(search.data(), &3);
    assert_eq!(search.template(), "/<*path>/raw");
    assert_eq!(search.parameters(), &[("path", "docs")]);

    Ok(())
}

#[test]
fn wildcard_inline_segment() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<*path>/<file>", 1)?;
    builder.insert("/<*path>.tar", 2)?;
    builder.insert("/<*path>/log", 3)?;

    let router = builder.build();

    let search = router.search("/var/log/app").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<*path>/<file>");
    assert_eq!(search.parameters(), &[("path", "var/log"), ("file", "app")]);

    let search = router.search("/var//log/app").unwrap();
    assert_eq!(search.data(), &1);
    assert_eq!(search.template(), "/<*path>/<file>");
    assert_eq!(search.parameters(), &[
        ("path", "var//log"),
        ("file", "app")
    ]);

    Ok(())
}
