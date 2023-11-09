//! Benchmark adapted from the shredder crate, released under MIT license. SArc: https://github.com/Others/shredder/blob/266de5a3775567463ee82febc42eed1c9a8b6197/benches/shredder_benchmark.rs

use std::hint::black_box;
use std::sync::{Arc, Weak};
use criterion::BenchmarkGroup;
use criterion::measurement::Measurement;

// BENCHMARK 3: Same as benchmark 2, but with parent pointers. Added by rust-cc

fn count_binary_trees_with_parent(max_size: usize) -> Vec<usize> {
    let mut res = Vec::new();
    {
        let min_size = 4;

        for depth in (min_size..max_size).step_by(2) {
            let iterations = 1 << (max_size - depth + min_size);
            let mut check = 0;

            for _ in 1..=iterations {
                check += (TreeNodeWithParent::new(depth)).check();
            }

            res.push(check);
        }
    }
    res
}

enum TreeNodeWithParent {
    Root {
        left: Arc<TreeNodeWithParent>,
        right: Arc<TreeNodeWithParent>,
    },
    Nested {
        parent: Weak<TreeNodeWithParent>,
        left: Arc<TreeNodeWithParent>,
        right: Arc<TreeNodeWithParent>,
    },
    End,
}

impl TreeNodeWithParent {
    fn new(depth: usize) -> Arc<Self> {
        if depth == 0 {
            return Arc::new(Self::End);
        }

        Arc::new_cyclic(|weak| Self::Root {
            left: Self::new_nested(depth - 1, weak),
            right: Self::new_nested(depth - 1, weak),
        })
    }

    fn new_nested(depth: usize, parent: &Weak<Self>) -> Arc<Self> {
        if depth == 0 {
            return Arc::new(Self::End);
        }

        Arc::new_cyclic(|weak| Self::Nested {
            parent: parent.clone(),
            left: Self::new_nested(depth - 1, weak),
            right: Self::new_nested(depth - 1, weak),
        })
    }

    fn check(&self) -> usize {
        match self {
            Self::Root { left, right } => left.check() + right.check() + 1,
            Self::Nested { left, right, parent } => {
                black_box(parent); // Use the parent field at least once in the code to avoid it being optimized out
                left.check() + right.check()
            },
            Self::End => 1,
        }
    }
}

pub fn benchmark_count_binary_trees_with_parent(c: &mut BenchmarkGroup<impl Measurement>) {
    c.bench_function("arc", |b| {
        b.iter(|| count_binary_trees_with_parent(black_box(11)))
    });
}
