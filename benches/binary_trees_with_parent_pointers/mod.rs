use criterion::Criterion;

mod rust_cc;
mod gc;
#[cfg(feature = "shredder")]
mod shredder;
mod broom;
mod mgc;
mod bacon_rajan_cc;

pub fn binary_trees_with_parent_pointers(c: &mut Criterion) {
    let group = &mut c.benchmark_group("binary trees with parent pointers");
    rust_cc::benchmark_count_binary_trees_with_parent(group);
    gc::benchmark_count_binary_trees_with_parent(group);
    #[cfg(feature = "shredder")]
    shredder::benchmark_count_binary_trees_with_parent(group);
    broom::benchmark_count_binary_trees_with_parent(group);
    mgc::benchmark_count_binary_trees_with_parent(group);
    bacon_rajan_cc::benchmark_count_binary_trees_with_parent(group);
}
