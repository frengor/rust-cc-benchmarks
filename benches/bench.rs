use std::time::Duration;
use criterion::{Criterion, criterion_group, criterion_main};

#[cfg(all(feature = "jemalloc", not(target_env = "msvc")))]
use tikv_jemallocator::Jemalloc;

#[cfg(all(feature = "jemalloc", not(target_env = "msvc")))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[allow(dead_code)]
pub(crate) static RUST_CC_BENCH_NAME: &str = if cfg!(feature = "rust-cc-finalization") {
    "rust-cc"
} else {
    "rust-cc (no fin.)"
};

#[cfg(feature = "stress-test")]
mod stress_test;
#[cfg(feature = "binary-trees")]
mod binary_trees;
#[cfg(feature = "binary-trees-with-parent-pointers")]
mod binary_trees_with_parent_pointers;
#[cfg(feature = "large-linked-list")]
mod large_linked_list;

macro_rules! cfg_benchmark {
    ($module:tt::$target:ident, $($cfg:tt)*) => {
        #[cfg($($cfg)*)]
        use $module::$target;

        #[cfg(not($($cfg)*))]
        fn $target(_: &mut Criterion) {}
    };
}

cfg_benchmark!(stress_test::stress_test, feature = "stress-test");
cfg_benchmark!(binary_trees::binary_trees, feature = "binary-trees");
cfg_benchmark!(binary_trees_with_parent_pointers::binary_trees_with_parent_pointers, feature = "binary-trees-with-parent-pointers");
cfg_benchmark!(large_linked_list::large_linked_list, feature = "large-linked-list");

criterion_group!(
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(10));
    targets =
        stress_test,
        binary_trees,
        binary_trees_with_parent_pointers,
        large_linked_list,
);

criterion_main!(benches);
