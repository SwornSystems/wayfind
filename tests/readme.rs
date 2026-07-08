#![expect(missing_docs, reason = "Tests")]

use core::error::Error;

use similar_asserts::assert_eq;
use wayfind::RouterBuilder;

#[test]
fn readme() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/v2", "end-1")?;
    builder.insert("/v2/<*name>/blobs/<algorithm>:<hash>", "end-2")?;
    builder.insert("/v2/<*name>/manifests/<reference>", "end-3")?;
    builder.insert("/v2/<*name>/blobs/uploads", "end-4a")?;
    builder.insert("/v2/<*name>/blobs/uploads/<reference>", "end-5")?;
    builder.insert("/v2/<*name>/tags/list", "end-8a")?;
    builder.insert("/v2/<*name>/referrers/<algorithm>:<hash>", "end-12a")?;

    let router = builder.build();
    insta::assert_snapshot!(router, @"
    /v2
    ╰─ /
       ╰─ <*name>
          ╰─ /
             ├─ blobs/
             │  ├─ uploads
             │  │  ╰─ /
             │  │     ╰─ <reference>
             │  ╰─ <algorithm>
             │     ╰─ :
             │        ╰─ <hash>
             ├─ manifests/
             │  ╰─ <reference>
             ├─ referrers/
             │  ╰─ <algorithm>
             │     ╰─ :
             │        ╰─ <hash>
             ╰─ tags/list
    ");

    let search = router.search("/v2").unwrap();
    assert_eq!(search.data(), &"end-1");
    assert_eq!(search.parameters(), &[]);

    let search = router
        .search("/v2/myorg/myrepo/blobs/sha256:2c26b46b68ff")
        .unwrap();

    assert_eq!(search.data(), &"end-2");
    assert_eq!(search.parameters(), &[
        ("name", "myorg/myrepo"),
        ("algorithm", "sha256"),
        ("hash", "2c26b46b68ff"),
    ]);

    let search = router.search("/v2/myorg/myrepo/manifests/latest").unwrap();

    assert_eq!(search.data(), &"end-3");
    assert_eq!(search.parameters(), &[
        ("name", "myorg/myrepo"),
        ("reference", "latest")
    ]);

    let search = router.search("/v2/myorg/myrepo/blobs/uploads").unwrap();

    assert_eq!(search.data(), &"end-4a");
    assert_eq!(search.parameters(), &[("name", "myorg/myrepo")]);

    let search = router
        .search("/v2/myorg/myrepo/blobs/uploads/e361beb4-576f")
        .unwrap();

    assert_eq!(search.data(), &"end-5");
    assert_eq!(search.parameters(), &[
        ("name", "myorg/myrepo"),
        ("reference", "e361beb4-576f")
    ]);

    let search = router.search("/v2/myorg/myrepo/tags/list").unwrap();

    assert_eq!(search.data(), &"end-8a");
    assert_eq!(search.parameters(), &[("name", "myorg/myrepo")]);

    let search = router
        .search("/v2/myorg/myrepo/referrers/sha256:2c26b46b68ff")
        .unwrap();

    assert_eq!(search.data(), &"end-12a");
    assert_eq!(search.parameters(), &[
        ("name", "myorg/myrepo"),
        ("algorithm", "sha256"),
        ("hash", "2c26b46b68ff"),
    ]);

    Ok(())
}
