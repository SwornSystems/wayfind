#![expect(
    missing_docs,
    clippy::missing_asserts_for_indexing,
    clippy::missing_panics_doc,
    clippy::unwrap_used,
    reason = "Bench"
)]

use core::fmt::Write as _;
use core::hint::black_box;

use divan::AllocProfiler;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

fn segments(count: usize) -> String {
    "/x".repeat(count)
}

fn anchors(count: usize) -> String {
    let mut path = "/x/-".repeat(count);
    path.push_str("/x");
    path
}

fn inline(count: usize) -> String {
    let mut path = String::from("/");
    for index in 0..count {
        if index > 0 {
            path.push('.');
        }

        path.push('x');
    }

    path
}

#[divan::bench(consts = [1, 10, 100, 1000])]
fn dynamic_competing<const N: usize>(bencher: divan::Bencher<'_, '_>) {
    let mut builder = wayfind::RouterBuilder::new();
    for index in 0..N {
        let template = format!("/<a>.z{index}");
        builder.insert(&template, index).unwrap();
    }

    let router = builder.build();
    let path = String::from("/x.zz");
    bencher.bench(|| black_box(router.search(black_box(path.as_str()))));
}

#[divan::bench(consts = [1, 10, 100, 1000])]
fn dynamic_depth<const N: usize>(bencher: divan::Bencher<'_, '_>) {
    let mut builder = wayfind::RouterBuilder::new();
    builder.insert("/<a>/z", 1).unwrap();

    let router = builder.build();
    let path = segments(N);
    bencher.bench(|| black_box(router.search(black_box(path.as_str()))));
}

#[divan::bench(consts = [1, 10, 100, 1000])]
fn dynamic_rescan<const N: usize>(bencher: divan::Bencher<'_, '_>) {
    let mut builder = wayfind::RouterBuilder::new();
    builder.insert("/<x>a<y>a<z>/-/<s>/end/<t>", 1).unwrap();

    let router = builder.build();
    let path = format!("/{}/-/{}/end/", "a".repeat(N), "b".repeat(N));
    bencher.bench(|| black_box(router.search(black_box(path.as_str()))));
}

#[divan::bench(consts = [1, 10, 100, 1000])]
fn dynamic_nested<const N: usize>(bencher: divan::Bencher<'_, '_>) {
    let mut builder = wayfind::RouterBuilder::new();
    builder.insert("/<a>.<b>.z", 1).unwrap();
    builder.insert("/<a>.<b>.y.<*c>", 2).unwrap();

    let router = builder.build();
    let path = format!("/{}.MISS", "z.".repeat(N));
    bencher.bench(|| black_box(router.search(black_box(path.as_str()))));
}

#[divan::bench(consts = [1, 10, 100, 1000])]
fn dynamic_inline<const N: usize>(bencher: divan::Bencher<'_, '_>) {
    let mut builder = wayfind::RouterBuilder::new();

    let mut template = String::from("/");
    for index in 0..N {
        if index > 0 {
            template.push('.');
        }

        let _unused = write!(template, "<a{index}>");
    }

    builder.insert(&template, 1).unwrap();

    let router = builder.build();
    let path = inline(N);
    bencher.bench(|| black_box(router.search(black_box(path.as_str()))));
}

#[divan::bench(consts = [1, 10, 100, 1000])]
fn wildcard_anchored<const N: usize>(bencher: divan::Bencher<'_, '_>) {
    let mut builder = wayfind::RouterBuilder::new();
    builder.insert("/<*a>/-/<*b>/x", 1).unwrap();

    let router = builder.build();
    let path = anchors(N);
    bencher.bench(|| black_box(router.search(black_box(path.as_str()))));
}

#[divan::bench(consts = [1, 10, 100, 1000])]
fn wildcard_backtrack<const N: usize>(bencher: divan::Bencher<'_, '_>) {
    let mut builder = wayfind::RouterBuilder::new();
    builder.insert("/<*a>/-/<*b>/x", 1).unwrap();

    let router = builder.build();
    let mut path = "/x/-".repeat(N);
    path.push_str("/y");

    bencher.bench(|| black_box(router.search(black_box(path.as_str()))));
}

#[divan::bench(consts = [1, 10, 100, 1000])]
fn wildcard_chain<const N: usize>(bencher: divan::Bencher<'_, '_>) {
    let mut builder = wayfind::RouterBuilder::new();

    let mut template = String::new();
    for index in 0..N {
        if index > 0 {
            template.push_str("/-");
        }

        let _unused = write!(template, "/<*a{index}>");
    }

    template.push_str("/x");
    builder.insert(&template, 1).unwrap();

    let router = builder.build();
    let path = anchors(N);
    bencher.bench(|| black_box(router.search(black_box(path.as_str()))));
}

#[divan::bench(consts = [1, 10, 100, 1000])]
fn wildcard_competing<const N: usize>(bencher: divan::Bencher<'_, '_>) {
    let mut builder = wayfind::RouterBuilder::new();
    for index in 0..N {
        let template = format!("/<*a>/z{index}");
        builder.insert(&template, index).unwrap();
    }

    let router = builder.build();
    let mut path = segments(20);
    path.push_str("/zz");

    bencher.bench(|| black_box(router.search(black_box(path.as_str()))));
}

#[divan::bench(consts = [1, 10, 100, 1000])]
fn wildcard_depth<const N: usize>(bencher: divan::Bencher<'_, '_>) {
    let mut builder = wayfind::RouterBuilder::new();
    builder.insert("/<*a>/z", 1).unwrap();

    let router = builder.build();
    let path = segments(N);
    bencher.bench(|| black_box(router.search(black_box(path.as_str()))));
}

#[divan::bench(consts = [1, 10, 100, 1000])]
fn wildcard_endings<const N: usize>(bencher: divan::Bencher<'_, '_>) {
    let mut builder = wayfind::RouterBuilder::new();
    builder.insert("/<*a>/x", 1).unwrap();
    builder.insert("/<*a>/y", 2).unwrap();
    builder.insert("/<*a>/z", 3).unwrap();

    let router = builder.build();
    let path = format!("{}/miss", "/x".repeat(N));
    bencher.bench(|| black_box(router.search(black_box(path.as_str()))));
}

#[divan::bench(consts = [1, 10, 100, 1000])]
fn wildcard_nested<const N: usize>(bencher: divan::Bencher<'_, '_>) {
    let mut builder = wayfind::RouterBuilder::new();
    builder.insert("/<*a>/x/<*b>/x", 1).unwrap();
    builder.insert("/<*a>/x/<*b>/y/<*c>", 2).unwrap();

    let router = builder.build();
    let path = format!("{}/z", "/x".repeat(N));
    bencher.bench(|| black_box(router.search(black_box(path.as_str()))));
}

#[divan::bench(consts = [1, 10, 100, 1000])]
fn wildcard_rescan<const N: usize>(bencher: divan::Bencher<'_, '_>) {
    let mut builder = wayfind::RouterBuilder::new();
    builder.insert("/<*a>/x/<*b>/y/<c>", 1).unwrap();

    let router = builder.build();
    let path = format!("{}/", "/x/y".repeat(N));
    bencher.bench(|| black_box(router.search(black_box(path.as_str()))));
}

#[divan::bench(consts = [1, 10, 100, 1000])]
fn wildcard_nested_inline<const N: usize>(bencher: divan::Bencher<'_, '_>) {
    let mut builder = wayfind::RouterBuilder::new();
    builder.insert("/<*a>.x.<*b>.x", 1).unwrap();
    builder.insert("/<*a>.x.<*b>.y.<*c>", 2).unwrap();

    let router = builder.build();
    let path = format!("/{}.z", ".x".repeat(N));
    bencher.bench(|| black_box(router.search(black_box(path.as_str()))));
}

#[divan::bench(consts = [1, 10, 100, 1000])]
fn wildcard_nested_present<const N: usize>(bencher: divan::Bencher<'_, '_>) {
    let mut builder = wayfind::RouterBuilder::new();
    builder.insert("/<*a>/x/<*b>/x", 1).unwrap();
    builder.insert("/<*a>/x/<*b>/y/<*c>", 2).unwrap();

    let router = builder.build();
    let path = format!("/y{}/z", "/x".repeat(N));
    bencher.bench(|| black_box(router.search(black_box(path.as_str()))));
}

#[divan::bench(consts = [1, 10, 100, 1000])]
fn wildcard_nested_triple<const N: usize>(bencher: divan::Bencher<'_, '_>) {
    let mut builder = wayfind::RouterBuilder::new();
    builder.insert("/<*a>/x/<*b>/x/<*c>/x", 1).unwrap();
    builder.insert("/<*a>/x/<*b>/x/<*c>/y/<*d>", 2).unwrap();

    let router = builder.build();
    let path = format!("{}/z", "/x".repeat(N));
    bencher.bench(|| black_box(router.search(black_box(path.as_str()))));
}

#[divan::bench(consts = [1, 10, 100, 1000])]
fn wildcard_open<const N: usize>(bencher: divan::Bencher<'_, '_>) {
    let mut builder = wayfind::RouterBuilder::new();
    builder.insert("/<*a>/x", 1).unwrap();
    builder.insert("/<*a>/y/<*b>", 2).unwrap();

    let router = builder.build();
    let path = format!("{}/miss", "/x".repeat(N));
    bencher.bench(|| black_box(router.search(black_box(path.as_str()))));
}

#[divan::bench(consts = [1, 10, 100, 1000])]
fn wildcard_suffixes<const N: usize>(bencher: divan::Bencher<'_, '_>) {
    let mut builder = wayfind::RouterBuilder::new();
    builder.insert("/<*a>.html", 1).unwrap();
    builder.insert("/<*a>.htm", 2).unwrap();

    let router = builder.build();
    let path = format!("/{}.txt", ".htm".repeat(N));
    bencher.bench(|| black_box(router.search(black_box(path.as_str()))));
}
