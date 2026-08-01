![license: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)
[![crates.io](https://img.shields.io/crates/v/wayfind)](https://crates.io/crates/wayfind)
[![documentation](https://docs.rs/wayfind/badge.svg)](https://docs.rs/wayfind)

![rust: 1.85+](https://img.shields.io/badge/rust-1.85+-orange.svg)
![`unsafe`: forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)
![`wasm`: compatible](https://img.shields.io/badge/wasm-compatible-success.svg)
![`no-std`: compatible](https://img.shields.io/badge/no--std-compatible-success.svg)

[![codecov](https://codecov.io/gh/SwornSystems/wayfind/graph/badge.svg)](https://app.codecov.io/gh/SwornSystems/wayfind)
[![codspeed](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://app.codspeed.io/SwornSystems/wayfind)

# `wayfind`

A speedy, flexible router for Rust.

## Why another router

Real-world projects often need advanced routing: inline parameters, mid-route wildcards, or compatibility with frameworks such as [Ruby on Rails](https://guides.rubyonrails.org/routing.html) and specifications such as the [OCI Distribution Specification](https://github.com/opencontainers/distribution-spec/blob/main/spec.md).

`wayfind` aims to be competitive with the fastest routers while supporting these features. Unused features don't impact performance.

## Showcase

```toml
[dependencies]
wayfind = "*"
```

```rust
use std::error::Error;

use wayfind::RouterBuilder;

let mut builder = RouterBuilder::new();
builder.insert("/v2", "end-1")?;
builder.insert("/v2/<*name>/blobs/<algorithm>:<hash>", "end-2")?;
builder.insert("/v2/<*name>/manifests/<reference>", "end-3")?;
builder.insert("/v2/<*name>/blobs/uploads", "end-4a")?;
builder.insert("/v2/<*name>/blobs/uploads/<reference>", "end-5")?;
builder.insert("/v2/<*name>/tags/list", "end-8a")?;
builder.insert("/v2/<*name>/referrers/<algorithm>:<hash>", "end-12a")?;

let router = builder.build();

let search = router.search("/v2").unwrap();
assert_eq!(search.data(), &"end-1");
assert_eq!(search.parameters(), &[]);

let search = router.search("/v2/myorg/myrepo/blobs/sha256:2c26b46b68ff").unwrap();
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
    ("reference", "latest"),
]);

let search = router.search("/v2/myorg/myrepo/blobs/uploads").unwrap();
assert_eq!(search.data(), &"end-4a");
assert_eq!(search.parameters(), &[
    ("name", "myorg/myrepo"),
]);

let search = router.search("/v2/myorg/myrepo/blobs/uploads/e361beb4-576f").unwrap();
assert_eq!(search.data(), &"end-5");
assert_eq!(search.parameters(), &[
    ("name", "myorg/myrepo"),
    ("reference", "e361beb4-576f"),
]);

let search = router.search("/v2/myorg/myrepo/tags/list").unwrap();
assert_eq!(search.data(), &"end-8a");
assert_eq!(search.parameters(), &[
    ("name", "myorg/myrepo"),
]);

let search = router.search("/v2/myorg/myrepo/referrers/sha256:2c26b46b68ff").unwrap();
assert_eq!(search.data(), &"end-12a");
assert_eq!(search.parameters(), &[
    ("name", "myorg/myrepo"),
    ("algorithm", "sha256"),
    ("hash", "2c26b46b68ff"),
]);

println!("{router}");
Ok::<_, Box<dyn Error>>(())
```

```text
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
```

## Implementation Details

`wayfind` stores routes in a compressed radix trie.

When searching, each node tries its children in priority order:
1. static
2. dynamic
3. wildcard

All parameters are greedy, consuming as much of the path as possible.

### Limitations

No backtracking happens across priority levels.
Some matches can surprise you.

In the following router:

```text
/api/
├─ <version>
│  ╰─ /
│     ╰─ <*rest>
╰─ <*path>
   ╰─ /help
```

The path `/api/docs/help` would match the first route, not the second.
Even though the second is arguably more specific.

## Performance

`wayfind` is competitive with the fastest Rust routers across all benchmarks we run.

For all benchmarks, we convert any extracted parameters to strings.

All routers offer a way to return parameters as strings, but some delay the actual UTF-8 decoding until post-search.

| Library          | Percent Decoding | String Parameters |
|:-----------------|:----------------:|:-----------------:|
| wayfind          | no               | yes               |
| actix-router     | partial          | yes               |
| matchit          | no               | delayed           |
| ntex-router      | partial          | yes               |
| path-tree        | no               | delayed           |
| route-recognizer | no               | yes               |
| xitca-router     | no               | yes               |

As such, we offer 2 sets of results per benchmark:
- one with the default behaviour of the router.
- one with the parameters extracted to `Vec<(&str, &str)>`.

See the results at: https://app.codspeed.io/SwornSystems/wayfind/benchmarks

## License

Licensed under the terms of both the [MIT License](LICENSE-MIT) and the [Apache License (Version 2.0)](LICENSE-APACHE).

## Inspirations

- [poem](https://github.com/poem-web/poem): Initial experimentations started out as a Poem router fork
- [matchit](https://github.com/ibraheemdev/matchit): Performance leader among pre-existing routers
- [path-tree](https://github.com/viz-rs/path-tree): Extensive testing and router display feature
