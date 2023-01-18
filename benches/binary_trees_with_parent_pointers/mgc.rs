//! Benchmark adapted from the shredder crate, released under MIT license. Src: https://github.com/Others/shredder/blob/266de5a3775567463ee82febc42eed1c9a8b6197/benches/shredder_benchmark.rs

use std::hint::black_box;
use criterion::BenchmarkGroup;
use criterion::measurement::Measurement;

use mgc::*;
use parking_lot::Mutex;

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
    sweep();
    res
}

enum TreeNodeWithParent {
    Root {
        left: Gc<TreeNodeWithParent>,
        right: Gc<TreeNodeWithParent>,
    },
    Nested {
        parent: Mutex<Option<Gc<TreeNodeWithParent>>>,
        left: Gc<TreeNodeWithParent>,
        right: Gc<TreeNodeWithParent>,
    },
    End,
}

impl Trace for TreeNodeWithParent {
    fn trace(&self) {
        match self {
            Self::Root { left, right } => {
                left.trace();
                right.trace();
            }
            Self::Nested { parent, left, right } => {
                parent.trace();
                left.trace();
                right.trace();
            }
            Self::End => {},
        }
    }
}

impl TreeNodeWithParent {
    fn new(depth: usize) -> Gc<Self> {
        if depth == 0 {
            return Gc::new(Self::End);
        }

        let root = Gc::new(Self::Root {
            left: Self::new_nested(depth - 1),
            right: Self::new_nested(depth - 1),
        });

        if let Self::Root{ left, right } = &*root {
            if let Self::Nested { parent, ..} = &**left {
                *parent.lock() = Some(root.clone());
            }

            if let Self::Nested { parent, ..} = &**right {
                *parent.lock() = Some(root.clone());
            }
        }

        root
    }

    fn new_nested(depth: usize) -> Gc<Self> {
        if depth == 0 {
            return Gc::new(Self::End);
        }

        let gc = Gc::new(Self::Nested {
            left: Self::new_nested(depth - 1),
            right: Self::new_nested(depth - 1),
            parent: Mutex::new(None),
        });

        if let Self::Nested{ left, right, .. } = &*gc {
            if let Self::Nested { parent, ..} = &**left {
                *parent.lock() = Some(gc.clone());
            }

            if let Self::Nested { parent, ..} = &**right {
                *parent.lock() = Some(gc.clone());
            }
        }

        gc
    }

    fn check(&self) -> usize {
        match self {
            Self::Root { left, right, .. }
            | Self::Nested { left, right, .. } => left.check() + right.check() + 1,
            Self::End => 1,
        }
    }
}

pub fn benchmark_count_binary_trees_with_parent(c: &mut BenchmarkGroup<impl Measurement>) {
    c.bench_function("mgc", |b| {
        b.iter(|| count_binary_trees_with_parent(black_box(11)))
    });
}
