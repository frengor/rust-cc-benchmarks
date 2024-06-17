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

pub fn large_linked_list(c: &mut Criterion) {
    let group = &mut c.benchmark_group("large linked list");
    rc::benchmark_large_linked_list(group);
    arc::benchmark_large_linked_list(group);
    rust_cc::benchmark_large_linked_list(group);
    gc::benchmark_large_linked_list(group);
    bacon_rajan_cc::benchmark_large_linked_list(group);
    safe_gc::benchmark_large_linked_list(group);
    #[cfg(feature = "zb-safe-gc")]
    zb_safe_gc::benchmark_large_linked_list(group);
    #[cfg(feature = "shredder")]
    shredder::benchmark_large_linked_list(group);
    #[cfg(feature = "broom")]
    broom::benchmark_large_linked_list(group);
    #[cfg(feature = "mgc")]
    mgc::benchmark_large_linked_list(group);
}
