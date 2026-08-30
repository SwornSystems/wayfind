#![expect(
    missing_docs,
    clippy::missing_panics_doc,
    clippy::unwrap_used,
    reason = "Bench"
)]

use core::hint::black_box;

use divan::AllocProfiler;

#[path = "fixtures/gitlab_routes.rs"]
mod gitlab_routes;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

const HITS: &[&str] = &[
    "/help",
    "/dashboard/projects",
    "/admin/application_settings",
    "/api/graphql",
    "/users/torvalds",
    "/users/torvalds/activity",
    "/groups/gitlab-org/-/group_members/42",
    "/-/ide/project/15/merge_requests/9",
];

const MISSES: &[&str] = &[
    "/.env",
    "/wp-login.php",
    "/-/health",
    "/api/v4/projects/1/repository/files/app.rb",
    "/gitlab-org/gitlab/-/merge_requests/2025/nonexistent/subpage",
    "/-/admin/dashboard/does/not/exist",
    "/users/torvalds/settings/does-not-exist",
    "/some/random/deep/path/that/nobody/requests",
];

fn main() {
    divan::main();
}

fn router() -> wayfind::Router<usize> {
    let mut builder = wayfind::RouterBuilder::new();
    for (index, route) in gitlab_routes::routes().iter().enumerate() {
        builder.insert(route, index).unwrap();
    }

    builder.build()
}

#[divan::bench]
fn gitlab_search_hits(bencher: divan::Bencher<'_, '_>) {
    let router = router();
    bencher.bench(|| {
        for path in black_box(HITS) {
            black_box(router.search(black_box(path)));
        }
    });
}

#[divan::bench]
fn gitlab_search_misses(bencher: divan::Bencher<'_, '_>) {
    let router = router();
    bencher.bench(|| {
        for path in black_box(MISSES) {
            black_box(router.search(black_box(path)));
        }
    });
}

#[divan::bench]
fn gitlab_insert(bencher: divan::Bencher<'_, '_>) {
    bencher.bench(|| {
        let mut builder = wayfind::RouterBuilder::new();
        for (index, route) in gitlab_routes::routes().iter().enumerate() {
            builder.insert(black_box(route), index).unwrap();
        }

        black_box(builder.build())
    });
}

#[divan::bench]
fn gitlab_display(bencher: divan::Bencher<'_, '_>) {
    let mut builder = wayfind::RouterBuilder::new();
    for (index, route) in gitlab_routes::routes().iter().enumerate() {
        builder.insert(route, index).unwrap();
    }

    let router = builder.build();
    bencher.bench(|| black_box(format!("{router}")));
}
