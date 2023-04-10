use std::hint::black_box;
use criterion::BenchmarkGroup;
use criterion::measurement::Measurement;

use mgc::*;
use parking_lot::Mutex;

fn large_linked_list(size: usize) -> Vec<usize> {
    let mut res = Vec::new();
    for _ in 0..30 {
        let mut list = List::new();
        for _ in 0..size {
            list.add();
        }
        res.push(list.len());
    }
    sweep();
    res
}

struct List {
    head: Gc<Node>,
}

impl List {
    fn new() -> List {
        List {
            head: Gc::new(Node::Nil),
        }
    }

    fn add(&mut self) {
        let cons = Gc::new(Node::Cons {
            next: self.head.clone(),
            previous: Mutex::new(None),
        });
        if let Node::Cons{ previous, .. } = &*self.head {
            *previous.lock() = Some(cons.clone());
        }
        self.head = cons;
    }

    fn len(&self) -> usize {
        self.head.len()
    }
}

enum Node {
    Cons { next: Gc<Node>, previous: Mutex<Option<Gc<Node>>> },
    Nil,
}

impl Node {
    fn len(&self) -> usize {
        match self {
            Self::Cons { next, .. } => {
                next.len() + 1
            },
            _ => 0,
        }
    }
}

impl Trace for Node {
    fn trace(&self) {
        match self {
            Self::Cons { next, previous } => {
                next.trace();
                previous.trace();
            },
            Self::Nil => {},
        }
    }
}

pub fn benchmark_large_linked_list(c: &mut BenchmarkGroup<impl Measurement>) {
    c.bench_function("mgc", |b| {
        b.iter(|| large_linked_list(black_box(4096)))
    });
}
