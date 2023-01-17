use criterion::Criterion;

mod rust_cc;
mod gc;
mod shredder;
mod comet;

pub fn binary_trees_with_parent_pointers(c: &mut Criterion) {
    let group = &mut c.benchmark_group("binary trees with parent pointers");
    rust_cc::benchmark_count_binary_trees_with_parent(group);
    gc::benchmark_count_binary_trees_with_parent(group);
}
