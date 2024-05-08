use criterion::Criterion;

mod rc;
mod arc;
mod rust_cc;
mod gc;
#[cfg(feature = "safe-gc")]
mod safe_gc;
#[cfg(feature = "shredder")]
mod shredder;
#[cfg(feature = "broom")]
mod broom;
mod mgc;
mod bacon_rajan_cc;

pub fn binary_trees_with_parent_pointers(c: &mut Criterion) {
    let group = &mut c.benchmark_group("binary trees with parent pointers");
    rc::benchmark_count_binary_trees_with_parent(group);
    arc::benchmark_count_binary_trees_with_parent(group);
    rust_cc::benchmark_count_binary_trees_with_parent(group);
    gc::benchmark_count_binary_trees_with_parent(group);
    #[cfg(feature = "safe-gc")]
    safe_gc::benchmark_count_binary_trees_with_parent(group);
    #[cfg(feature = "shredder")]
    shredder::benchmark_count_binary_trees_with_parent(group);
    #[cfg(feature = "broom")]
    broom::benchmark_count_binary_trees_with_parent(group);
    mgc::benchmark_count_binary_trees_with_parent(group);
    bacon_rajan_cc::benchmark_count_binary_trees_with_parent(group);
}
