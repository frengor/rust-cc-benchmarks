use criterion::Criterion;

#[cfg(feature = "rc")]
mod rc;
#[cfg(feature = "arc")]
mod arc;
#[cfg(feature = "rust-cc")]
mod rust_cc;
#[cfg(feature = "gc")]
mod gc;
#[cfg(feature = "safe-gc")]
mod safe_gc;
#[cfg(feature = "zb-safe-gc")]
mod zb_safe_gc;
#[cfg(feature = "shredder")]
mod shredder;
#[cfg(feature = "broom")]
mod broom;
#[cfg(feature = "mgc")]
mod mgc;
#[cfg(feature = "bacon-rajan-cc")]
mod bacon_rajan_cc;

pub fn binary_trees_with_parent_pointers(c: &mut Criterion) {
    let group = &mut c.benchmark_group("binary trees with parent pointers");
    #[cfg(feature = "rc")]
    rc::benchmark_count_binary_trees_with_parent(group);
    #[cfg(feature = "arc")]
    arc::benchmark_count_binary_trees_with_parent(group);
    #[cfg(feature = "rust-cc")]
    rust_cc::benchmark_count_binary_trees_with_parent(group);
    #[cfg(feature = "gc")]
    gc::benchmark_count_binary_trees_with_parent(group);
    #[cfg(feature = "bacon-rajan-cc")]
    bacon_rajan_cc::benchmark_count_binary_trees_with_parent(group);
    #[cfg(feature = "safe-gc")]
    safe_gc::benchmark_count_binary_trees_with_parent(group);
    #[cfg(feature = "zb-safe-gc")]
    zb_safe_gc::benchmark_count_binary_trees_with_parent(group);
    #[cfg(feature = "shredder")]
    shredder::benchmark_count_binary_trees_with_parent(group);
    #[cfg(feature = "broom")]
    broom::benchmark_count_binary_trees_with_parent(group);
    #[cfg(feature = "mgc")]
    mgc::benchmark_count_binary_trees_with_parent(group);
}
