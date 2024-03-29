use criterion::Criterion;

mod rust_cc;
mod gc;
#[cfg(feature = "shredder")]
mod shredder;
mod broom;
mod mgc;
mod bacon_rajan_cc;

pub fn binary_trees(c: &mut Criterion) {
    let group = &mut c.benchmark_group("binary trees");
    rust_cc::benchmark_count_binary_trees(group);
    gc::benchmark_count_binary_trees(group);
    #[cfg(feature = "shredder")]
    shredder::benchmark_count_binary_trees(group);
    broom::benchmark_count_binary_trees(group);
    mgc::benchmark_count_binary_trees(group);
    bacon_rajan_cc::benchmark_count_binary_trees(group);
}
