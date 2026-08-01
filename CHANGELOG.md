# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.0](https://github.com/SwornSystems/wayfind/compare/v1.0.3...v1.1.0) - 2026-08-01

### Features
- Rename DuskSystems to SwornSystems ([0a9a891](https://github.com/SwornSystems/wayfind/commit/0a9a8919572de4273b475d5d2521a59d7e88b15a))

## [1.0.3](https://github.com/SwornSystems/wayfind/compare/v1.0.2...v1.0.3) - 2026-06-13

### Fixes
- Correct suffix boundaries order ([d338916](https://github.com/SwornSystems/wayfind/commit/d338916b0ea3d67dc84ad6790e4d4bb0458dcfbb))

### Performance
- Only memoize revisitable parameters ([4579fda](https://github.com/SwornSystems/wayfind/commit/4579fdaeead2b62b510a156328ea5468acc483a8))

### Testing
- Add inline wildcard coverage ([0536003](https://github.com/SwornSystems/wayfind/commit/0536003f9bb38b9d3b4bf6f0d3509b1e91ea099c))

## [1.0.2](https://github.com/SwornSystems/wayfind/compare/v1.0.1...v1.0.2) - 2026-06-10

### Fixes
- Correct handling of overlapping suffixes ([91030c7](https://github.com/SwornSystems/wayfind/commit/91030c774dfe1b3c544c6d83a599c00266991df8))

### Performance
- Replace SmallVec with custom storage ([3680375](https://github.com/SwornSystems/wayfind/commit/368037517939c282ef4f106026089b939c1c49db))
- Use smallvec for caps ([2e926f7](https://github.com/SwornSystems/wayfind/commit/2e926f7b878f7e909135c8b8a52359ef24e74228))
- Index search memos by ID ([0820f14](https://github.com/SwornSystems/wayfind/commit/0820f146e6fd1c53640369031baabc7a3cb66ee3))

### Testing
- Add overlapping suffix edge case ([3e51a65](https://github.com/SwornSystems/wayfind/commit/3e51a6556291fa295998ebcdbb461092f371b800))

## [1.0.1](https://github.com/SwornSystems/wayfind/compare/v1.0.0...v1.0.1) - 2026-05-11

### Performance
- Smarter reachability pruning ([9b29851](https://github.com/SwornSystems/wayfind/commit/9b2985168a31671218f8bb59a063a86c028211e7))

## [1.0.0](https://github.com/SwornSystems/wayfind/compare/v0.9.1...v1.0.0) - 2026-04-04

### Other

- Bump major version

## [0.9.1](https://github.com/SwornSystems/wayfind/compare/v0.9.0...v0.9.1) - 2026-04-04

### Added

- Introduce builder for router
- Track visited nodes
- Use path pruning and suffix matching during searches
- Add GitLab benches

### Fixed

- Skip invalid chars early
- Correct boundary check for wildcards
- Avoid UTF-8 costs during search

### Other

- Split up builder, compiler and router
- Flatten error type
- Avoid exposing smallvec in API
- Remove get functionality
- Remove delete functionality
- Tweak static search approach
- Prune non reachable
- Make use of `memchr` for string searching

## [0.9.0] - 2025-08-27

### Added

- Support `no-std` builds.
- `Router::get` and `Router::get_mut` methods to access data from pre-existing templates.

### Changed

- Raised MSRV from 1.63 to 1.85.
- Switch to Rust 2024 edition.
- Syntax for parameters has changed from `{name}` to `<name>`, to remove ambiguity with formatted strings.
- Parameter characters `<` and `>` can no longer be escaped, since they aren't valid URL characters anyways.
- Display no longer shows nodes storing data, since it was redundant.

### Removed

- Constraints are no longer supported.
- Optional groups are no longer supported.
- `Parameters` alias has been removed.

### Fixed

- Improved detection of structural conflicts.
- Corrected routing priority for inline parameters.

## [0.8.1] - 2025-01-07

### Fixed

- Remove required lifetime from router.

## [0.8.0] - 2025-01-07

### Changed

- The router no longer performs percent-decoding.
- Searches no longer error on any invalid UTF-8 parameters, instead acting as if the path doesn't match instead.
- Error messages are now more consistent.
- The term 'route' has been replaced with 'template'.
- Lowered MSRV from 1.66 to 1.63.

### Removed

- The `Routable` wrapper is no longer needed. The router uses strings for inserts.
- The `Path` wrapper is no longer needed. The router uses strings for searches.
- `EncodingError` has been removed.
- `PathError` has been removed.
- `RoutableError` has been removed.

### Fixed

- Priority logic now correctly prefers depth, then length, of templates.

## [0.7.0] - 2024-11-12

### Added

- Support builder pattern for constructing routables.

### Changed

- `Router` now requires an explicit lifetime.
- All routes now require a leading `/`.
- Router display no longer shows internal root node. Matchable nodes are now denoted by `[*]`.
- Route expander now converts empty routes `""` to `"/"`.
- `Parameters` lists make use of `smallvec` now.

### Removed

- Individual parameters are returned as tuples, rather than `Parameter`.

### Fixed

- Touching parameters are now correctly rejected.

## [0.6.0] - 2024-10-27

### Added

- Added support for optional groups in routes.
- Added support for inline wildcards.

### Changed

- Inserts and deletes should be much more efficient now.
- Syntax for escaping parameters has changed to use the `\` character.
- Route errors now have more consistent error messages.
- Duplicate route error now shows which route caused the conflict.
- Added priority ranking to routes.

### Removed

- Optional parameters have been replaced with optional groups.
- Optional trailing slashes have been replaced with optional groups.

## [0.5.0] - 2024-09-11

### Fixed

- Ensure routes can only be deleted via the exact same inserted route.

### Changed

- Router display is more compact now.

## [0.4.0] - 2024-09-11

### Added

- Optional trailing slashes are now supported.

### Fixed

- Route error messages are now correctly indented.

## [0.3.0] - 2024-09-11

### Added

- Optional parameters are now supported in routes.

### Changed

- Successful matches now return a flattened representation of node data.
- Route encoding errors now live in the encoding error enum.
- Router display now uses different characters to represent root and matchable nodes.

### Fixed

- Router delete method now checks for encoded routes.
- Be consistent with the use of terms "path" and "route".

## [0.2.1] - 2024-09-04

### Changed

- Added OCI example.

### Fixed

- Router display no longer relies on generic being displayable.

## [0.2.0] - 2024-08-29

### Changed

- Search method now returns a result.

### Removed

- Removed all usages of unsafe code.
- Removed smallvec dependency.

### Security

- Resolved incorrect unsafe usage in parameter extraction.

## [0.1.0] - 2024-08-29

### Added

- Initial implementation of `wayfind`.

[0.9.0]: https://github.com/SwornSystems/wayfind/compare/v0.8.1...v0.9.0
[0.8.1]: https://github.com/SwornSystems/wayfind/compare/v0.8.0...v0.8.1
[0.8.0]: https://github.com/SwornSystems/wayfind/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/SwornSystems/wayfind/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/SwornSystems/wayfind/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/SwornSystems/wayfind/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/SwornSystems/wayfind/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/SwornSystems/wayfind/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/SwornSystems/wayfind/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/SwornSystems/wayfind/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/SwornSystems/wayfind/releases/tag/v0.1.0
