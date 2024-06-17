use criterion::Criterion;

mod rc;
mod arc;
mod rust_cc;
mod gc;
mod safe_gc;
#[cfg(feature = "zb-safe-gc")]
mod zb_safe_gc;
#[cfg(feature = "shredder")]
mod shredder;
#[cfg(feature = "broom")]
mod broom;
#[cfg(feature = "mgc")]
mod mgc;
mod bacon_rajan_cc;

pub fn binary_trees(c: &mut Criterion) {
    let group = &mut c.benchmark_group("binary trees");
    rc::benchmark_count_binary_trees(group);
    arc::benchmark_count_binary_trees(group);
    rust_cc::benchmark_count_binary_trees(group);
    gc::benchmark_count_binary_trees(group);
    bacon_rajan_cc::benchmark_count_binary_trees(group);
    safe_gc::benchmark_count_binary_trees(group);
    #[cfg(feature = "zb-safe-gc")]
    zb_safe_gc::benchmark_count_binary_trees(group);
    #[cfg(feature = "shredder")]
    shredder::benchmark_count_binary_trees(group);
    #[cfg(feature = "broom")]
    broom::benchmark_count_binary_trees(group);
    #[cfg(feature = "mgc")]
    mgc::benchmark_count_binary_trees(group);
}
