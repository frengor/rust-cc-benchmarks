//! Benchmark adapted from the shredder crate, released under MIT license. Src: https://github.com/Others/shredder/blob/266de5a3775567463ee82febc42eed1c9a8b6197/benches/shredder_benchmark.rs

use std::hint::black_box;
use criterion::BenchmarkGroup;
use criterion::measurement::Measurement;

use cgc_single_threaded::api::*;
use cgc_single_threaded::heap::Heap;

// BENCHMARK 2: It's binary-trees from the benchmarks game!

fn count_binary_trees(max_size: usize) -> Vec<usize> {
    let mut heap = Heap::new(1024, 2048, false);
    let mut res = Vec::new();
    {
        let min_size = 4;

        for depth in (min_size..max_size).step_by(2) {
            let iterations = 1 << (max_size - depth + min_size);
            let mut check = 0;

            for _ in 1..=iterations {
                let tree = TreeNode::new(depth, &mut heap);
                check += heap.allocate(tree).check();
            }

            res.push(check);
        }
    }
    heap.collect();
    res
}

enum TreeNode {
    Nested {
        left: Handle<TreeNode>,
        right: Handle<TreeNode>,
    },
    End,
}

impl Traceable for TreeNode {
    fn trace_with(&self, tracer: &mut Tracer) {
        if let Self::Nested { left, right } = self {
            left.trace_with(tracer);
            right.trace_with(tracer);
        }
    }
}

impl Finalizer for TreeNode {}

impl TreeNode {
    fn new(depth: usize, heap: &mut Heap) -> Self {
        if depth == 0 {
            return Self::End;
        }

        let left = Self::new(depth - 1, heap);
        let right = Self::new(depth - 1, heap);
        Self::Nested {
            left: heap.allocate(left).to_heap(),
            right: heap.allocate(right).to_heap(),
        }
    }

    fn check(&self) -> usize {
        match self {
            Self::End => 1,
            Self::Nested { left, right } => left.check() + right.check() + 1,
        }
    }
}

pub fn benchmark_count_binary_trees(c: &mut BenchmarkGroup<impl Measurement>) {
    c.bench_function("cgc-single-threaded", |b| {
        b.iter(|| count_binary_trees(black_box(11)))
    });
}
