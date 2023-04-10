use std::cell::RefCell;
use std::hint::black_box;
use criterion::BenchmarkGroup;
use criterion::measurement::Measurement;

use shredder::*;

fn large_linked_list(size: usize) -> Vec<usize> {
    let mut res = Vec::new();
    for _ in 0..30 {
        let mut list = List::new();
        for _ in 0..size {
            list.add();
        }
        res.push(list.len());
    }
    collect();
    res
}

#[derive(Scan)]
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
            previous: RefCell::new(None),
        });
        if let Node::Cons{ previous, .. } = &*self.head.get() {
            *previous.borrow_mut() = Some(cons.clone());
        }
        self.head = cons;
    }

    fn len(&self) -> usize {
        self.head.get().len()
    }
}

#[derive(Scan)]
enum Node {
    Cons { next: Gc<Node>, previous: RefCell<Option<Gc<Node>>> },
    Nil,
}

impl Node {
    fn len(&self) -> usize {
        match self {
            Self::Cons { next, .. } => {
                next.get().len() + 1
            },
            _ => 0,
        }
    }
}

pub fn benchmark_large_linked_list(c: &mut BenchmarkGroup<impl Measurement>) {
    c.bench_function("shredder", |b| {
        b.iter(|| large_linked_list(black_box(4096)))
    });
}
