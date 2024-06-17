use std::hint::black_box;
use criterion::BenchmarkGroup;
use criterion::measurement::Measurement;

use safe_gc::*;

fn large_linked_list(size: usize) -> Vec<usize> {
    let mut heap = Heap::new();
    let mut res = Vec::new();
    for _ in 0..30 {
        let mut list = List::new(&mut heap);
        for _ in 0..size {
            list.add(&mut heap);
        }
        res.push(list.len(&mut heap));
    }
    heap.gc();
    res
}

struct List {
    head: Root<Node>,
}

impl List {
    fn new(heap: &mut Heap) -> List {
        List {
            head: heap.alloc(Node::Nil),
        }
    }

    fn add(&mut self, heap: &mut Heap) {
        let cons = heap.alloc(Node::Cons {
            next: self.head.unrooted(),
            previous: None,
        });
        if let Node::Cons{ previous, .. } = heap.get_mut(&self.head) {
            *previous = Some(cons.unrooted());
        }
        self.head = cons;
    }

    fn len(&self, heap: &Heap) -> usize {
        heap.get(&self.head).len(heap)
    }
}

enum Node {
    Cons { next: Gc<Node>, previous: Option<Gc<Node>> },
    Nil,
}

impl Node {
    fn len(&self, heap: &Heap) -> usize {
        match self {
            Self::Cons { next, .. } => {
                heap[*next].len(heap) + 1
            },
            _ => 0,
        }
    }
}

impl Trace for Node {
    fn trace(&self, collector: &mut Collector) {
        match self {
            Self::Cons { next, previous } => {
                collector.edge(*next);
                if let Some(previous) = *previous {
                    collector.edge(previous);
                }
            },
            Self::Nil => {},
        }
    }
}

pub fn benchmark_large_linked_list(c: &mut BenchmarkGroup<impl Measurement>) {
    c.bench_function("safe-gc", |b| {
        b.iter_with_large_drop(|| large_linked_list(black_box(4096)))
    });
}
