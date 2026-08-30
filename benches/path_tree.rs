#![expect(
    missing_docs,
    clippy::let_underscore_must_use,
    clippy::missing_panics_doc,
    clippy::unwrap_used,
    reason = "Bench"
)]

use core::hint::black_box;

use divan::AllocProfiler;
use routes::paths;

#[path = "fixtures/path_tree_routes.rs"]
pub mod routes;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

#[divan::bench(name = "wayfind")]
fn wayfind_default(bencher: divan::Bencher<'_, '_>) {
    let mut builder = wayfind::RouterBuilder::new();
    for (index, route) in routes!(arrows).iter().enumerate() {
        builder.insert(route, index).unwrap();
    }

    let router = builder.build();
    bencher.bench(|| {
        for path in black_box(paths()) {
            let _output = black_box(router.search(black_box(path)).unwrap());
        }
    });
}

#[divan::bench(name = "wayfind_parameters")]
fn wayfind_parameters(bencher: divan::Bencher<'_, '_>) {
    let mut builder = wayfind::RouterBuilder::new();
    for (index, route) in routes!(arrows).iter().enumerate() {
        builder.insert(route, index).unwrap();
    }

    let router = builder.build();
    bencher.bench(|| {
        for path in black_box(paths()) {
            let output = black_box(router.search(black_box(path)).unwrap());
            let _parameters: Vec<(&str, &str)> =
                black_box(output.parameters().iter().map(|p| (p.0, p.1)).collect());
        }
    });
}

#[divan::bench(name = "actix_router")]
fn actix_router_default(bencher: divan::Bencher<'_, '_>) {
    let mut router = actix_router::Router::<usize>::build();
    for (index, route) in routes!(brackets).iter().enumerate() {
        router.path(*route, index);
    }
    let router = router.finish();

    bencher.bench(|| {
        for path in black_box(paths()) {
            let mut path = actix_router::Path::new(path);
            let _output = black_box(router.recognize(black_box(&mut path)).unwrap());
        }
    });
}

#[divan::bench(name = "actix_router_parameters")]
fn actix_router_parameters(bencher: divan::Bencher<'_, '_>) {
    let mut router = actix_router::Router::<usize>::build();
    for (index, route) in routes!(brackets).iter().enumerate() {
        router.path(*route, index);
    }
    let router = router.finish();

    bencher.bench(|| {
        for path in black_box(paths()) {
            let mut path = actix_router::Path::new(path);
            let _output = black_box(router.recognize(black_box(&mut path)).unwrap());
            let _parameters: Vec<(&str, &str)> =
                black_box(path.iter().map(|p| (p.0, p.1)).collect());
        }
    });
}

#[divan::bench(name = "matchit")]
fn matchit_default(bencher: divan::Bencher<'_, '_>) {
    let mut router = matchit::Router::new();
    for (index, route) in routes!(brackets).iter().enumerate() {
        router.insert(*route, index).unwrap();
    }

    bencher.bench(|| {
        for path in black_box(paths()) {
            let _output = black_box(router.at(black_box(path)).unwrap());
        }
    });
}

#[divan::bench(name = "matchit_parameters")]
fn matchit_parameters(bencher: divan::Bencher<'_, '_>) {
    let mut router = matchit::Router::new();
    for (index, route) in routes!(brackets).iter().enumerate() {
        router.insert(*route, index).unwrap();
    }

    bencher.bench(|| {
        for path in black_box(paths()) {
            let output = black_box(router.at(black_box(path)).unwrap());
            let _parameters: Vec<(&str, &str)> =
                black_box(output.params.iter().map(|p| (p.0, p.1)).collect());
        }
    });
}

#[divan::bench(name = "ntex_router")]
fn ntex_router_default(bencher: divan::Bencher<'_, '_>) {
    let mut router = ntex_router::Router::<usize>::build();
    for (index, route) in routes!(brackets).iter().enumerate() {
        router.path(*route, index);
    }
    let router = router.finish();

    bencher.bench(|| {
        for path in black_box(paths()) {
            let mut path = ntex_router::Path::new(path);
            let _output = black_box(router.recognize(black_box(&mut path)).unwrap());
        }
    });
}

#[divan::bench(name = "ntex_router_parameters")]
fn ntex_router_parameters(bencher: divan::Bencher<'_, '_>) {
    let mut router = ntex_router::Router::<usize>::build();
    for (index, route) in routes!(brackets).iter().enumerate() {
        router.path(*route, index);
    }
    let router = router.finish();

    bencher.bench(|| {
        for path in black_box(paths()) {
            let mut path = ntex_router::Path::new(path);
            let _output = black_box(router.recognize(black_box(&mut path)).unwrap());
            let _parameters: Vec<(&str, &str)> =
                black_box(path.iter().map(|p| (p.0, p.1)).collect());
        }
    });
}

#[divan::bench(name = "path_tree")]
fn path_tree_default(bencher: divan::Bencher<'_, '_>) {
    let mut router = path_tree::PathTree::new();
    for (index, route) in routes!(colon).iter().enumerate() {
        let _ = router.insert(route, index);
    }

    bencher.bench(|| {
        for path in black_box(paths()) {
            let _output = black_box(router.find(path).unwrap());
        }
    });
}

#[divan::bench(name = "path_tree_parameters")]
fn path_tree_parameters(bencher: divan::Bencher<'_, '_>) {
    let mut router = path_tree::PathTree::new();
    for (index, route) in routes!(colon).iter().enumerate() {
        let _ = router.insert(route, index);
    }

    bencher.bench(|| {
        for path in black_box(paths()) {
            let output = router.find(path).unwrap();
            let _parameters: Vec<(&str, &str)> =
                black_box(output.1.params_iter().map(|p| (p.0, p.1)).collect());
        }
    });
}

#[divan::bench(name = "route_recognizer")]
fn route_recognizer_default(bencher: divan::Bencher<'_, '_>) {
    let mut router = route_recognizer::Router::<usize>::new();
    for (index, route) in routes!(colon).iter().enumerate() {
        router.add(route, index);
    }

    bencher.bench(|| {
        for path in black_box(paths()) {
            let _output = black_box(router.recognize(path).unwrap());
        }
    });
}

#[divan::bench(name = "route_recognizer_parameters")]
fn route_recognizer_parameters(bencher: divan::Bencher<'_, '_>) {
    let mut router = route_recognizer::Router::<usize>::new();
    for (index, route) in routes!(colon).iter().enumerate() {
        router.add(route, index);
    }

    bencher.bench(|| {
        for path in black_box(paths()) {
            let output = router.recognize(path).unwrap();
            let _parameters: Vec<(&str, &str)> =
                black_box(output.params().iter().map(|p| (p.0, p.1)).collect());
        }
    });
}

#[divan::bench(name = "xitca_router")]
fn xitca_router_default(bencher: divan::Bencher<'_, '_>) {
    let mut router = xitca_router::Router::new();
    for (index, route) in routes!(brackets).iter().enumerate() {
        router.insert(*route, index).unwrap();
    }

    bencher.bench(|| {
        for path in black_box(paths()) {
            let _output = black_box(router.at(path).unwrap());
        }
    });
}

#[divan::bench(name = "xitca_router_parameters")]
fn xitca_router_parameters(bencher: divan::Bencher<'_, '_>) {
    let mut router = xitca_router::Router::new();
    for (index, route) in routes!(brackets).iter().enumerate() {
        router.insert(*route, index).unwrap();
    }

    bencher.bench(|| {
        for path in black_box(paths()) {
            let output = router.at(path).unwrap();
            let _parameters: Vec<(&str, &str)> =
                black_box(output.params.iter().map(|p| (p.0, p.1)).collect());
        }
    });
}
