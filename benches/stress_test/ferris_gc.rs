//! Benchmark adapted from the shredder crate, released under MIT license. Src: https://github.com/Others/shredder/blob/266de5a3775567463ee82febc42eed1c9a8b6197/benches/shredder_benchmark.rs

use std::hint::black_box;
use criterion::BenchmarkGroup;
use criterion::measurement::Measurement;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;

use ferris_gc::*;

// BENCHMARK 1: My janky stress test
// (It basically creates a graph where every node is rooted, then de-roots some nodes a few at a time)
struct DirectedGraphNode {
    _label: String,
    edges: Vec<Gc<GcCell<DirectedGraphNode>>>,
}

// Manually implementing since proc macro prints debug stuff on stdout
impl Trace for DirectedGraphNode {
    fn is_root(&self) -> bool {
        unreachable!("is_root should never be called on user-defined type !!")
    }
    fn reset_root(&self) {
        self._label.reset_root();
        self.edges.reset_root();
    }
    fn trace(&self) {
        self._label.trace();
        self.edges.trace();
    }
    fn reset(&self) {
        self._label.reset();
        self.edges.reset();
    }
    fn is_traceable(&self) -> bool {
        unreachable!("is_traceable should never be called on user-defined type !!")
    }
}

impl Finalize for DirectedGraphNode {
    fn finalize(&self) {}
}

const NODE_COUNT: usize = 1 << 15;
const EDGE_COUNT: usize = 1 << 15;
const SHRINK_DIV: usize = 1 << 10;

fn stress_test(rng: &mut StdRng) -> Vec<usize> {
    let mut res = Vec::new();
    {
        let mut nodes = Vec::new();

        for i in 0..=NODE_COUNT {
            nodes.push(Gc::new(GcCell::new(DirectedGraphNode {
                _label: format!("Node {}", i),
                edges: Vec::new(),
            })));
        }

        for _ in 0..=EDGE_COUNT {
            let a = nodes.choose(rng).unwrap();
            let b = nodes.choose(rng).unwrap();

            a.borrow_mut().edges.push(Gc::clone(b));
        }

        for i in 0..NODE_COUNT {
            if i % SHRINK_DIV == 0 {
                nodes.truncate(NODE_COUNT - i);
                let _ = ApplicationCleanup;
                res.push(nodes.len());
            }
        }
    }
    let _ = ApplicationCleanup;
    res
}

pub fn benchmark_stress_test(c: &mut BenchmarkGroup<impl Measurement>, rng: &mut StdRng) {
    c.bench_function("ferris-gc", |b| b.iter(|| stress_test(black_box(rng))));
}
