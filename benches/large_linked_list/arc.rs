use std::cell::RefCell;
use std::hint::black_box;
use std::sync::{Arc, Weak};

use criterion::measurement::Measurement;
use criterion::BenchmarkGroup;

fn large_linked_list(size: usize) -> Vec<usize> {
    let mut res = Vec::new();
    for _ in 0..30 {
        let mut list = List::new();
        for _ in 0..size {
            list.add();
        }
        res.push(list.len());
    }
    res
}

struct List {
    head: Arc<Node>,
}

impl List {
    fn new() -> List {
        List {
            head: Arc::new(Node::Nil),
        }
    }

    fn add(&mut self) {
        let cons = Arc::new(Node::Cons {
            next: self.head.clone(),
            previous: RefCell::new(None),
        });
        if let Node::Cons{ previous, .. } = &*self.head {
            *previous.borrow_mut() = Some(Arc::downgrade(&cons));
        }
        self.head = cons;
    }

    fn len(&self) -> usize {
        self.head.len()
    }
}

enum Node {
    Cons { next: Arc<Node>, previous: RefCell<Option<Weak<Node>>> },
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

pub fn benchmark_large_linked_list(c: &mut BenchmarkGroup<impl Measurement>) {
    c.bench_function("arc", |b| {
        b.iter_with_large_drop(|| large_linked_list(black_box(4096)))
    });
}
