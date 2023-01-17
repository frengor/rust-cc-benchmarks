use criterion::Criterion;

mod rust_cc;
mod gc;
mod shredder;
mod comet;

pub fn binary_trees(c: &mut Criterion) {
    let group = &mut c.benchmark_group("binary trees");
    rust_cc::benchmark_count_binary_trees(group);
    gc::benchmark_count_binary_trees(group);
}
