//! Benchmark adapted from the shredder crate, released under MIT license. Src: https://github.com/Others/shredder/blob/266de5a3775567463ee82febc42eed1c9a8b6197/benches/shredder_benchmark.rs

use std::cell::RefCell;
use std::hint::black_box;
use criterion::BenchmarkGroup;
use criterion::measurement::Measurement;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;

use rust_cc::*;

// BENCHMARK 1: My janky stress test
// (It basically creates a graph where every node is rooted, then de-roots some nodes a few at a time)
struct DirectedGraphNode {
    _label: String,
    edges: Vec<Cc<RefCell<DirectedGraphNode>>>,
}

unsafe impl Trace for DirectedGraphNode {
    fn trace(&self, ctx: &mut Context<'_>) {
        self.edges.iter().for_each(|elem| elem.trace(ctx));
    }
}

impl Finalize for DirectedGraphNode {}

const NODE_COUNT: usize = 1 << 15;
const EDGE_COUNT: usize = 1 << 15;
const SHRINK_DIV: usize = 1 << 10;

fn stress_test(rng: &mut StdRng) -> Vec<usize> {
    let mut res = Vec::new();
    {
        let mut nodes = Vec::new();

        for i in 0..=NODE_COUNT {
            nodes.push(Cc::new(RefCell::new(DirectedGraphNode {
                _label: format!("Node {}", i),
                edges: Vec::new(),
            })));
        }

        for _ in 0..=EDGE_COUNT {
            let a = nodes.choose(rng).unwrap();
            let b = nodes.choose(rng).unwrap();

            a.borrow_mut().edges.push(Cc::clone(b));
        }

        for i in 0..NODE_COUNT {
            if i % SHRINK_DIV == 0 {
                nodes.truncate(NODE_COUNT - i);
                collect_cycles();
                res.push(nodes.len());
            }
        }
    }
    collect_cycles();
    res
}

pub fn benchmark_stress_test(c: &mut BenchmarkGroup<impl Measurement>, rng: &mut StdRng) {
    c.bench_function("rust-cc", |b| b.iter_with_large_drop(|| stress_test(black_box(rng))));
}
