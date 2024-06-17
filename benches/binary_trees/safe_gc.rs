//! Benchmark adapted from the shredder crate, released under MIT license. Src: https://github.com/Others/shredder/blob/266de5a3775567463ee82febc42eed1c9a8b6197/benches/shredder_benchmark.rs

use std::hint::black_box;
use criterion::BenchmarkGroup;
use criterion::measurement::Measurement;

use safe_gc::*;

// BENCHMARK 2: It's binary-trees from the benchmarks game!

fn count_binary_trees(max_size: usize) -> Vec<usize> {
    let mut heap = Heap::new();
    let mut res = Vec::new();
    {
        let min_size = 4;

        for depth in (min_size..max_size).step_by(2) {
            let iterations = 1 << (max_size - depth + min_size);
            let mut check = 0;

            for _ in 1..=iterations {
                let root = TreeNode::new(depth, &mut heap);
                check += heap[&root].check(&heap);
            }

            res.push(check);
        }
    }
    heap.gc();
    res
}

enum TreeNode {
    Nested {
        left: Gc<TreeNode>,
        right: Gc<TreeNode>,
    },
    End,
}

impl Trace for TreeNode {
    fn trace(&self, collector: &mut Collector) {
        if let Self::Nested { left, right } = self {
            collector.edge(*left);
            collector.edge(*right);
        }
    }
}

impl TreeNode {
    fn new(depth: usize, heap: &mut Heap) -> Root<Self> {
        if depth == 0 {
            return heap.alloc(Self::End);
        }

        let left = Self::new(depth - 1, heap);
        let right = Self::new(depth - 1, heap);
        heap.alloc(Self::Nested {
            left: left.unrooted(),
            right: right.unrooted(),
        })
    }

    fn check(&self, heap: &Heap) -> usize {
        match self {
            Self::End => 1,
            Self::Nested { left, right } => heap[*left].check(heap) + heap[*right].check(heap) + 1,
        }
    }
}

pub fn benchmark_count_binary_trees(c: &mut BenchmarkGroup<impl Measurement>) {
    c.bench_function("safe-gc", |b| {
        b.iter_with_large_drop(|| count_binary_trees(black_box(11)))
    });
}
