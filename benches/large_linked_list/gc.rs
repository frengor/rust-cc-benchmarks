use std::hint::black_box;
use criterion::BenchmarkGroup;
use criterion::measurement::Measurement;

use gc::*;

fn large_linked_list(size: usize) -> Vec<usize> {
    let mut res = Vec::new();
    for _ in 0..30 {
        let mut list = List::new();
        for _ in 0..size {
            list.add();
        }
        res.push(list.len());
    }
    force_collect();
    res
}

#[derive(Trace, Finalize)]
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
            previous: GcCell::new(None),
        });
        if let Node::Cons{ previous, .. } = &*self.head {
            *previous.borrow_mut() = Some(cons.clone());
        }
        self.head = cons;
    }

    fn len(&self) -> usize {
        self.head.len()
    }
}

#[derive(Trace, Finalize)]
enum Node {
    Cons { next: Gc<Node>, previous: GcCell<Option<Gc<Node>>> },
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
    c.bench_function("gc", |b| {
        b.iter_with_large_drop(|| large_linked_list(black_box(4096)))
    });
}
