//! Benchmark adapted from the shredder crate, released under MIT license. Src: https://github.com/Others/shredder/blob/266de5a3775567463ee82febc42eed1c9a8b6197/benches/shredder_benchmark.rs

use std::hint::black_box;
use criterion::BenchmarkGroup;
use criterion::measurement::Measurement;

use safe_gc::*;

// BENCHMARK 3: Same as benchmark 2, but with parent pointers. Added by rust-cc

fn count_binary_trees_with_parent(max_size: usize) -> Vec<usize> {
    let mut heap = Heap::new();
    let mut res = Vec::new();
    {
        let min_size = 4;

        for depth in (min_size..max_size).step_by(2) {
            let iterations = 1 << (max_size - depth + min_size);
            let mut check = 0;

            for _ in 1..=iterations {
                let root = TreeNodeWithParent::new(depth, &mut heap);
                check += heap[root].check(&heap);
            }

            res.push(check);
        }
    }
    heap.gc();
    res
}

enum TreeNodeWithParent {
    Root {
        left: Gc<TreeNodeWithParent>,
        right: Gc<TreeNodeWithParent>,
    },
    Nested {
        parent: Option<Gc<TreeNodeWithParent>>,
        left: Gc<TreeNodeWithParent>,
        right: Gc<TreeNodeWithParent>,
    },
    End,
}

impl Trace for TreeNodeWithParent {
    fn trace(&self, collector: &mut Collector) {
        match self {
            Self::Root { left, right } => {
                collector.edge(*left);
                collector.edge(*right);
            }
            Self::Nested { parent, left, right } => {
                if let Some(parent) = *parent {
                    collector.edge(parent);
                }
                collector.edge(*left);
                collector.edge(*right);
            }
            Self::End => {},
        }
    }
}

impl TreeNodeWithParent {
    fn new(depth: usize, heap: &mut Heap) -> Root<Self> {
        if depth == 0 {
            return heap.alloc(Self::End);
        }

        let left = Self::new_nested(depth - 1, heap);
        let right = Self::new_nested(depth - 1, heap);

        let root = heap.alloc(Self::Root {
            left: left.unrooted(),
            right: right.unrooted(),
        });

        if let Self::Nested { parent, .. } = heap.get_mut(left) {
            *parent = Some(root.unrooted());
        }
        if let Self::Nested { parent, .. } = heap.get_mut(right) {
            *parent = Some(root.unrooted());
        }

        root
    }

    fn new_nested(depth: usize, heap: &mut Heap) -> Root<Self> {
        if depth == 0 {
            return heap.alloc(Self::End);
        }

        let left = Self::new_nested(depth - 1, heap);
        let right = Self::new_nested(depth - 1, heap);

        let nested = heap.alloc(Self::Nested {
            left: left.unrooted(),
            right: right.unrooted(),
            parent: None,
        });

        if let Self::Nested { parent: p, .. } = heap.get_mut(left) {
            *p = Some(nested.unrooted());
        }
        if let Self::Nested { parent: p, .. } = heap.get_mut(right) {
            *p = Some(nested.unrooted());
        }

        nested
    }

    fn check(&self, heap: &Heap) -> usize {
        match self {
            Self::Root { left, right, .. }
            | Self::Nested { left, right, .. } => heap[*left].check(heap) + heap[*right].check(heap) + 1,
            Self::End => 1,
        }
    }
}

pub fn benchmark_count_binary_trees_with_parent(c: &mut BenchmarkGroup<impl Measurement>) {
    c.bench_function("safe-gc", |b| {
        b.iter_with_large_drop(|| count_binary_trees_with_parent(black_box(11)))
    });
}
