//! Benchmark adapted from the shredder crate, released under MIT license. Src: https://github.com/Others/shredder/blob/266de5a3775567463ee82febc42eed1c9a8b6197/benches/shredder_benchmark.rs

use std::hint::black_box;
use criterion::BenchmarkGroup;
use criterion::measurement::Measurement;

use mgc::*;

// BENCHMARK 2: It's binary-trees from the benchmarks game!

fn count_binary_trees(max_size: usize) -> Vec<usize> {
    let mut res = Vec::new();
    {
        let min_size = 4;

        for depth in (min_size..max_size).step_by(2) {
            let iterations = 1 << (max_size - depth + min_size);
            let mut check = 0;

            for _ in 1..=iterations {
                check += Gc::new(TreeNode::new(depth)).check();
            }

            res.push(check);
        }
    }
    sweep();
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
    fn trace(&self) {
        if let Self::Nested { left, right } = self {
            left.trace();
            right.trace();
        }
    }
}

impl TreeNode {
    fn new(depth: usize) -> Self {
        if depth == 0 {
            return Self::End;
        }

        Self::Nested {
            left: Gc::new(Self::new(depth - 1)),
            right: Gc::new(Self::new(depth - 1)),
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
    c.bench_function("mgc", |b| {
        b.iter(|| count_binary_trees(black_box(11)))
    });
}
