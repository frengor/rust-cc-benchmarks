//! Benchmark adapted from the shredder crate, released under MIT license. Src: https://github.com/Others/shredder/blob/266de5a3775567463ee82febc42eed1c9a8b6197/benches/shredder_benchmark.rs

use std::hint::black_box;
use criterion::BenchmarkGroup;
use criterion::measurement::Measurement;

use cgc_single_threaded::api::*;
use cgc_single_threaded::heap::Heap;

use crate::utils::CgcRefCell;

// BENCHMARK 3: Same as benchmark 2, but with parent pointers. Added by rust-cc

fn count_binary_trees_with_parent(max_size: usize) -> Vec<usize> {
    let mut heap = Heap::new(1024, 2048, false);
    let mut res = Vec::new();
    {
        let min_size = 4;

        for depth in (min_size..max_size).step_by(2) {
            let iterations = 1 << (max_size - depth + min_size);
            let mut check = 0;

            for _ in 1..=iterations {
                check += (TreeNodeWithParent::new(depth, &mut heap)).check();
            }

            res.push(check);
        }
    }
    heap.collect();
    res
}

enum TreeNodeWithParent {
    Root {
        left: Handle<TreeNodeWithParent>,
        right: Handle<TreeNodeWithParent>,
    },
    Nested {
        parent: CgcRefCell<Option<Handle<TreeNodeWithParent>>>,
        left: Handle<TreeNodeWithParent>,
        right: Handle<TreeNodeWithParent>,
    },
    End,
}

impl Traceable for TreeNodeWithParent {
    fn trace_with(&self, tracer: &mut Tracer) {
        match self {
            Self::Root { left, right } => {
                left.trace_with(tracer);
                right.trace_with(tracer);
            }
            Self::Nested { parent, left, right } => {
                parent.trace_with(tracer);
                left.trace_with(tracer);
                right.trace_with(tracer);
            }
            Self::End => {},
        }
    }
}

impl Finalizer for TreeNodeWithParent {}

impl TreeNodeWithParent {
    fn new(depth: usize, heap: &mut Heap) -> Rooted<Self> {
        if depth == 0 {
            return heap.allocate(Self::End);
        }

        let root = Self::Root {
            left: Self::new_nested(depth - 1, heap),
            right: Self::new_nested(depth - 1, heap),
        };

        let root = heap.allocate(root);

        if let Self::Root{ left, right } = &*root {
            if let Self::Nested { parent, ..} = &**left {
                *parent.0.borrow_mut() = Some(root.to_heap());
            }

            if let Self::Nested { parent, ..} = &**right {
                *parent.0.borrow_mut() = Some(root.to_heap());
            }
        }

        root
    }

    fn new_nested(depth: usize, heap: &mut Heap) -> Handle<Self> {
        if depth == 0 {
            return heap.allocate(Self::End).to_heap();
        }

        let nested = Self::Nested {
            left: Self::new_nested(depth - 1, heap),
            right: Self::new_nested(depth - 1, heap),
            parent: CgcRefCell::new(None),
        };
        heap.allocate(nested).to_heap()
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
    c.bench_function("cgc-single-threaded", |b| {
        b.iter(|| count_binary_trees_with_parent(black_box(11)))
    });
}
