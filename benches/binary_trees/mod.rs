use criterion::Criterion;

mod rust_cc;
mod gc;
mod shredder;
mod cgc_single_threaded;
mod ferris_gc;

pub fn binary_trees(c: &mut Criterion) {
    let group = &mut c.benchmark_group("binary trees");
    rust_cc::benchmark_count_binary_trees(group);
    gc::benchmark_count_binary_trees(group);
    shredder::benchmark_count_binary_trees(group);
    cgc_single_threaded::benchmark_count_binary_trees(group);
    ferris_gc::benchmark_count_binary_trees(group);
}
