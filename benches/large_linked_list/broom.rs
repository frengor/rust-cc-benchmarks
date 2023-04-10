use std::hint::black_box;
use criterion::BenchmarkGroup;
use criterion::measurement::Measurement;

use broom::prelude::*;

fn large_linked_list(size: usize) -> Vec<usize> {
    let mut heap: Heap<Node> = Heap::default();
    let mut res = Vec::new();
    for _ in 0..30 {
        let mut list = List::new(&mut heap);
        for _ in 0..size {
            list.add(&mut heap);
        }
        res.push(list.len(&mut heap));
    }
    heap.clean();
    res
}

struct List {
    head: Rooted<Node>,
}

impl List {
    fn new(heap: &mut Heap<Node>) -> List {
        List {
            head: heap.insert(Node::Nil),
        }
    }

    fn add(&mut self, heap: &mut Heap<Node>) {
        let cons = heap.insert(Node::Cons {
            next: self.head.as_ref().clone(),
            previous: None,
        });
        if let Node::Cons{ previous, .. } = heap.get_mut(self.head.as_ref()).unwrap() {
            *previous = Some(cons.as_ref().clone());
        }
        self.head = cons;
    }

    fn len(&self, heap: &Heap<Node>) -> usize {
        heap.get(self.head.as_ref()).unwrap().len(heap)
    }
}

enum Node {
    Cons { next: Handle<Node>, previous: Option<Handle<Node>> },
    Nil,
}

impl Node {
    fn len(&self, heap: &Heap<Node>) -> usize {
        match self {
            Self::Cons { next, .. } => {
                heap.get(next).unwrap().len(heap) + 1
            },
            _ => 0,
        }
    }
}

impl Trace<Self> for Node {
    fn trace(&self, tracer: &mut Tracer<Self>) {
        match self {
            Self::Cons { next, previous } => {
                next.trace(tracer);
                if let Some(previous) = previous {
                    previous.trace(tracer);
                }
            },
            Self::Nil => {},
        }
    }
}

pub fn benchmark_large_linked_list(c: &mut BenchmarkGroup<impl Measurement>) {
    c.bench_function("broom", |b| {
        b.iter(|| large_linked_list(black_box(4096)))
    });
}
