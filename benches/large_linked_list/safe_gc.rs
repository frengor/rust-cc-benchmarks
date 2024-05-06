use criterion::measurement::Measurement;
use criterion::BenchmarkGroup;
use std::hint::black_box;

use safe_gc::*;

fn large_linked_list(size: usize) -> Vec<usize> {
    let mut res = Vec::new();
    for _ in 0..30 {
        let mut list = List::new();
        for _ in 0..size {
            list.add();
        }
        res.push(list.len());
    }
    Gc::collect();
    res
}

struct List {
    head: Gc<Node>,
}

impl Mark for List {
    fn mark_all(&self, generation: u64) {
        self.head.mark_all(generation);
    }

    fn unroot(&self) -> Mutability {
        self.head.unroot()
    }

    fn root(&self) {
        self.head.root();
    }

    fn destroy(&self) {
        self.head.destroy();
    }
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
            previous: GcCell::new(None),
        });
        if let Node::Cons { previous, .. } = &*self.head {
            *previous.borrow_mut() = Some(cons.clone());
        }
        self.head = cons;
    }

    fn len(&self) -> usize {
        self.head.len()
    }
}

enum Node {
    Cons {
        next: Gc<Node>,
        previous: GcCell<Option<Gc<Node>>>,
    },
    Nil,
}

impl Mark for Node {
    fn mark_all(&self, generation: u64) {
        match self {
            Node::Cons { next, previous } => {
                next.mark_all(generation);
                previous.mark_all(generation);
            },
            Node::Nil => {},
        }
    }

    fn unroot(&self) -> Mutability {
        match self {
            Node::Cons { next, previous } => next.unroot().or(&previous.unroot()),
            Node::Nil => Mutability::None,
        }
    }

    fn root(&self) {
        match self {
            Node::Cons { next, previous } => {
                next.root();
                previous.root();
            },
            Node::Nil => {},
        }
    }

    fn destroy(&self) {
        match self {
            Node::Cons { next, previous } => {
                next.destroy();
                previous.destroy();
            },
            Node::Nil => {},
        }
    }
}

impl Node {
    fn len(&self) -> usize {
        match self {
            Self::Cons { next, .. } => next.len() + 1,
            _ => 0,
        }
    }
}

pub fn benchmark_large_linked_list(c: &mut BenchmarkGroup<impl Measurement>) {
    c.bench_function("safe-gc", |b| b.iter(|| large_linked_list(black_box(4096))));
}
