use std::time::Duration;
use criterion::{Criterion, criterion_group, criterion_main};

#[cfg(all(feature = "jemalloc", not(target_env = "msvc")))]
use tikv_jemallocator::Jemalloc;

#[cfg(all(feature = "jemalloc", not(target_env = "msvc")))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

mod stress_test;
mod binary_trees;
mod binary_trees_with_parent_pointers;
mod large_linked_list;

criterion_group!(
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(10));
    targets =
        binary_trees_with_parent_pointers::binary_trees_with_parent_pointers,
        large_linked_list::large_linked_list,
        stress_test::stress_test,
        binary_trees::binary_trees
);
criterion_main!(benches);
