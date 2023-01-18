//! Benchmark adapted from the shredder crate, released under MIT license. Src: https://github.com/Others/shredder/blob/266de5a3775567463ee82febc42eed1c9a8b6197/benches/shredder_benchmark.rs

use std::hint::black_box;
use criterion::BenchmarkGroup;
use criterion::measurement::Measurement;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;

use broom::prelude::*;

// BENCHMARK 1: My janky stress test
// (It basically creates a graph where every node is rooted, then de-roots some nodes a few at a time)
struct DirectedGraphNode {
    _label: String,
    edges: Vec<Handle<DirectedGraphNode>>,
}

impl Trace<DirectedGraphNode> for DirectedGraphNode {
    fn trace(&self, tracer: &mut Tracer<DirectedGraphNode>) {
        self.edges.trace(tracer);
    }
}

const NODE_COUNT: usize = 1 << 15;
const EDGE_COUNT: usize = 1 << 15;
const SHRINK_DIV: usize = 1 << 10;

fn stress_test(rng: &mut StdRng) -> Vec<usize> {
    let mut heap = Heap::default();
    let mut res = Vec::new();
    {
        let mut nodes = Vec::new();

        for i in 0..=NODE_COUNT {
            nodes.push(heap.insert(DirectedGraphNode {
                _label: format!("Node {}", i),
                edges: Vec::new(),
            }));
        }

        for _ in 0..=EDGE_COUNT {
            let a = nodes.choose(rng).unwrap();
            let b = nodes.choose(rng).unwrap();

            heap.get_mut(a).unwrap().edges.push(b.handle());
        }

        for i in 0..NODE_COUNT {
            if i % SHRINK_DIV == 0 {
                nodes.truncate(NODE_COUNT - i);
                heap.clean();
                res.push(nodes.len());
            }
        }
    }
    heap.clean();
    res
}

pub fn benchmark_stress_test(c: &mut BenchmarkGroup<impl Measurement>, rng: &mut StdRng) {
    c.bench_function("broom", |b| b.iter(|| stress_test(black_box(rng))));
}
