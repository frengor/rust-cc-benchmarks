//! Benchmark adapted from the shredder crate, released under MIT license. Src: https://github.com/Others/shredder/blob/266de5a3775567463ee82febc42eed1c9a8b6197/benches/shredder_benchmark.rs

use std::hint::black_box;
use criterion::BenchmarkGroup;
use criterion::measurement::Measurement;

use safe_gc::*;

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
    Gc::collect();
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

impl Mark for TreeNodeWithParent {
    fn mark_all(&self, generation: u64) {
        match self {
            TreeNodeWithParent::Root { left, right } => {
                left.mark_all(generation);
                right.mark_all(generation);
            }
            TreeNodeWithParent::Nested { parent, left, right } => {
                parent.mark_all(generation);
                left.mark_all(generation);
                right.mark_all(generation);
            }
            TreeNodeWithParent::End => {}
        }
    }

    fn unroot(&self) -> Mutability {
        match self {
            TreeNodeWithParent::Root { left, right } => {
                left.unroot().or(&right.unroot())
            }
            TreeNodeWithParent::Nested { parent, left, right } => {
                parent.unroot().or(&left.unroot().or(&right.unroot()))
            }
            TreeNodeWithParent::End => {
                Mutability::None
            }
        }
    }

    fn root(&self) {
        match self {
            TreeNodeWithParent::Root { left, right } => {
                left.root();
                right.root();
            }
            TreeNodeWithParent::Nested { parent, left, right } => {
                parent.root();
                left.root();
                right.root();
            }
            TreeNodeWithParent::End => {}
        }
    }

    fn destroy(&self) {
        match self {
            TreeNodeWithParent::Root { left, right } => {
                left.destroy();
                right.destroy();
            }
            TreeNodeWithParent::Nested { parent, left, right } => {
                parent.destroy();
                left.destroy();
                right.destroy();

            }
            TreeNodeWithParent::End => {}
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
                *parent.borrow_mut() = Some(root.clone());
            }

            if let Self::Nested { parent, ..} = &**right {
                *parent.borrow_mut() = Some(root.clone());
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
            parent: GcCell::new(None),
        });

        if let Self::Nested{ left, right, .. } = &*gc {
            if let Self::Nested { parent, ..} = &**left {
                *parent.borrow_mut() = Some(gc.clone());
            }

            if let Self::Nested { parent, ..} = &**right {
                *parent.borrow_mut() = Some(gc.clone());
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
    c.bench_function("safe-gc", |b| {
        b.iter(|| count_binary_trees_with_parent(black_box(11)))
    });
}
