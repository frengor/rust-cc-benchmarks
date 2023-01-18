//! Benchmark adapted from the shredder crate, released under MIT license. Src: https://github.com/Others/shredder/blob/266de5a3775567463ee82febc42eed1c9a8b6197/benches/shredder_benchmark.rs

use std::hint::black_box;
use criterion::BenchmarkGroup;
use criterion::measurement::Measurement;

use broom::prelude::*;

// BENCHMARK 2: It's binary-trees from the benchmarks game!

fn count_binary_trees(max_size: usize) -> Vec<usize> {
    let mut heap = Heap::default();
    let mut res = Vec::new();
    {
        let min_size = 4;

        for depth in (min_size..max_size).step_by(2) {
            let iterations = 1 << (max_size - depth + min_size);
            let mut check = 0;

            for _ in 1..=iterations {
                let tree = TreeNode::new(depth, &mut heap);
                let root = heap.insert(tree);
                check += heap.get(root).unwrap().check(&heap);
            }

            res.push(check);
        }
    }
    heap.clean();
    res
}

enum TreeNode {
    Nested {
        left: Handle<TreeNode>,
        right: Handle<TreeNode>,
    },
    End,
}

impl Trace<Self> for TreeNode {
    fn trace(&self, tracer: &mut Tracer<Self>) {
        if let Self::Nested { left, right } = self {
            left.trace(tracer);
            right.trace(tracer);
        }
    }
}

impl TreeNode {
    fn new(depth: usize, heap: &mut Heap<TreeNode>) -> Self {
        if depth == 0 {
            return Self::End;
        }

        let left = Self::new(depth - 1, heap);
        let right = Self::new(depth - 1, heap);
        Self::Nested {
            left: heap.insert_temp(left),
            right: heap.insert_temp(right),
        }
    }

    fn check(&self, heap: &Heap<TreeNode>) -> usize {
        match self {
            Self::End => 1,
            Self::Nested { left, right } => heap.get(left).unwrap().check(heap) + heap.get(right).unwrap().check(heap) + 1,
        }
    }
}

pub fn benchmark_count_binary_trees(c: &mut BenchmarkGroup<impl Measurement>) {
    c.bench_function("broom", |b| {
        b.iter(|| count_binary_trees(black_box(11)))
    });
}
