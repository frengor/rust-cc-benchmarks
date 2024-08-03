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

pub fn large_linked_list(c: &mut Criterion) {
    let group = &mut c.benchmark_group("large linked list");
    #[cfg(feature = "rc")]
    rc::benchmark_large_linked_list(group);
    #[cfg(feature = "arc")]
    arc::benchmark_large_linked_list(group);
    #[cfg(feature = "rust-cc")]
    rust_cc::benchmark_large_linked_list(group);
    #[cfg(feature = "gc")]
    gc::benchmark_large_linked_list(group);
    #[cfg(feature = "bacon-rajan-cc")]
    bacon_rajan_cc::benchmark_large_linked_list(group);
    #[cfg(feature = "safe-gc")]
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
