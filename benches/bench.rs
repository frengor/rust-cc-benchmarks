use criterion::{criterion_group, criterion_main};

mod stress_test;
mod binary_trees;
mod binary_trees_with_parent_pointers;
mod large_linked_list;

criterion_group!(benches,
    stress_test::stress_test,
    binary_trees::binary_trees,
    binary_trees_with_parent_pointers::binary_trees_with_parent_pointers,
    large_linked_list::large_linked_list
);
criterion_main!(benches);
