use criterion::Criterion;

mod rust_cc;
mod gc;
#[cfg(feature = "shredder")]
mod shredder;
mod broom;
mod mgc;

pub fn large_linked_list(c: &mut Criterion) {
    let group = &mut c.benchmark_group("large linked list");
    rust_cc::benchmark_large_linked_list(group);
    gc::benchmark_large_linked_list(group);
    #[cfg(feature = "shredder")]
    shredder::benchmark_large_linked_list(group);
    broom::benchmark_large_linked_list(group);
    mgc::benchmark_large_linked_list(group);
}
