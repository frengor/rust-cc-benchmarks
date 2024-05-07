//! Benchmark adapted from the shredder crate, released under MIT license. Src: https://github.com/Others/shredder/blob/266de5a3775567463ee82febc42eed1c9a8b6197/benches/shredder_benchmark.rs

use std::hint::black_box;
use criterion::BenchmarkGroup;
use criterion::measurement::Measurement;

use broom::prelude::*;

// BENCHMARK 3: Same as benchmark 2, but with parent pointers. Added by rust-cc

fn count_binary_trees_with_parent(max_size: usize) -> Vec<usize> {
    let mut heap = Heap::default();
    let mut res = Vec::new();
    {
        let min_size = 4;

        for depth in (min_size..max_size).step_by(2) {
            let iterations = 1 << (max_size - depth + min_size);
            let mut check = 0;

            for _ in 1..=iterations {
                let root = TreeNodeWithParent::new(depth, &mut heap);
                check += heap.get(root).unwrap().check(&heap);
            }

            res.push(check);
        }
    }
    heap.clean();
    res
}

enum TreeNodeWithParent {
    Root {
        left: Handle<TreeNodeWithParent>,
        right: Handle<TreeNodeWithParent>,
    },
    Nested {
        parent: Option<Handle<TreeNodeWithParent>>,
        left: Handle<TreeNodeWithParent>,
        right: Handle<TreeNodeWithParent>,
    },
    End,
}

impl Trace<Self> for TreeNodeWithParent {
    fn trace(&self, tracer: &mut Tracer<Self>) {
        match self {
            Self::Root { left, right } => {
                left.trace(tracer);
                right.trace(tracer);
            }
            Self::Nested { parent, left, right } => {
                if let Some(parent) = *parent {
                    parent.trace(tracer);
                }
                left.trace(tracer);
                right.trace(tracer);
            }
            Self::End => {},
        }
    }
}

impl TreeNodeWithParent {
    fn new(depth: usize, heap: &mut Heap<TreeNodeWithParent>) -> Rooted<Self> {
        if depth == 0 {
            return heap.insert(Self::End);
        }

        let left = Self::new_nested(depth - 1, heap);
        let right = Self::new_nested(depth - 1, heap);

        let root = heap.insert(Self::Root {
            left,
            right,
        });

        if let Self::Nested { parent, .. } = heap.get_mut(left).unwrap() {
            *parent = Some(root.handle());
        }
        if let Self::Nested { parent, .. } = heap.get_mut(right).unwrap() {
            *parent = Some(root.handle());
        }

        root
    }

    fn new_nested(depth: usize, heap: &mut Heap<TreeNodeWithParent>) -> Handle<Self> {
        if depth == 0 {
            return heap.insert_temp(Self::End);
        }

        let left = Self::new_nested(depth - 1, heap);
        let right = Self::new_nested(depth - 1, heap);

        let nested = heap.insert_temp(Self::Nested {
            left,
            right,
            parent: None,
        });

        if let Self::Nested { parent: p, .. } = heap.get_mut(left).unwrap() {
            *p = Some(nested);
        }
        if let Self::Nested { parent: p, .. } = heap.get_mut(right).unwrap() {
            *p = Some(nested);
        }

        nested
    }

    fn check(&self, heap: &Heap<TreeNodeWithParent>) -> usize {
        match self {
            Self::Root { left, right, .. }
            | Self::Nested { left, right, .. } => heap.get(left).unwrap().check(heap) + heap.get(right).unwrap().check(heap) + 1,
            Self::End => 1,
        }
    }
}

pub fn benchmark_count_binary_trees_with_parent(c: &mut BenchmarkGroup<impl Measurement>) {
    c.bench_function("broom", |b| {
        b.iter_with_large_drop(|| count_binary_trees_with_parent(black_box(11)))
    });
}
