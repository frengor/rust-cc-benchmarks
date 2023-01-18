//! Benchmark adapted from the shredder crate, released under MIT license. Src: https://github.com/Others/shredder/blob/266de5a3775567463ee82febc42eed1c9a8b6197/benches/shredder_benchmark.rs

use std::hint::black_box;
use criterion::BenchmarkGroup;
use criterion::measurement::Measurement;

use ferris_gc::*;

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
    let _ = ApplicationCleanup;
    res
}

enum TreeNodeWithParent {
    Root {
        left: Gc<TreeNodeWithParent>,
        right: Gc<TreeNodeWithParent>,
    },
    Nested {
        parent: GcCell<Option<Gc<TreeNodeWithParent>>>,
        left: Gc<TreeNodeWithParent>,
        right: Gc<TreeNodeWithParent>,
    },
    End,
}

// Manually implementing since enums are not supported by proc macro
impl Trace for TreeNodeWithParent {
    fn is_root(&self) -> bool {
        unreachable!("is_root should never be called on user-defined type !!")
    }
    fn reset_root(&self) {
        match self {
            Self::Root { left, right } => {
                left.reset_root();
                right.reset_root();
            }
            Self::Nested { parent, left, right } => {
                parent.reset_root();
                left.reset_root();
                right.reset_root();
            }
            Self::End => {},
        }
    }
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
    fn reset(&self) {
        match self {
            Self::Root { left, right } => {
                left.reset();
                right.reset();
            }
            Self::Nested { parent, left, right } => {
                parent.reset();
                left.reset();
                right.reset();
            }
            Self::End => {},
        }
    }
    fn is_traceable(&self) -> bool {
        unreachable!("is_traceable should never be called on user-defined type !!")
    }
}

impl Finalize for TreeNodeWithParent {
    fn finalize(&self) {}
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

        if let Self::Root{ left, right } = &***root {
            if let Self::Nested { parent, ..} = &****left {
                **parent.borrow_mut() = Some(root.clone());
            }

            if let Self::Nested { parent, ..} = &****right {
                **parent.borrow_mut() = Some(root.clone());
            }
        }

        root
    }

    fn new_nested(depth: usize) -> Gc<Self> {
        if depth == 0 {
            return Gc::new(Self::End);
        }

        Gc::new(Self::Nested {
            left: Self::new_nested(depth - 1),
            right: Self::new_nested(depth - 1),
            parent: GcCell::new(None),
        })
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
    c.bench_function("ferris-gc", |b| {
        b.iter(|| count_binary_trees_with_parent(black_box(11)))
    });
}
