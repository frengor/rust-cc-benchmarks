use std::cell::RefCell;
use std::hint::black_box;

use criterion::measurement::Measurement;
use criterion::BenchmarkGroup;
use rust_cc::*;

fn large_linked_list(size: usize) -> Vec<usize> {
    let mut res = Vec::new();
    for _ in 0..30 {
        let mut list = List::new();
        for _ in 0..size {
            list.add();
        }
        res.push(list.len());
    }
    collect_cycles();
    res
}

struct List {
    head: Cc<Node>,
}

impl List {
    fn new() -> List {
        List {
            head: Cc::new(Node::Nil),
        }
    }

    fn add(&mut self) {
        let cons = Cc::new(Node::Cons {
            next: self.head.clone(),
            previous: RefCell::new(None),
        });
        if let Node::Cons{ previous, .. } = &*self.head {
            *previous.borrow_mut() = Some(cons.clone());
        }
        self.head = cons;
        #[cfg(feature = "rust-cc-mark-alive")]
        if let Node::Cons { next, .. } = &*self.head {
            next.mark_alive();
        }
    }

    fn len(&self) -> usize {
        self.head.len()
    }
}

#[derive(Trace, Finalize)]
enum Node {
    Cons { next: Cc<Node>, previous: RefCell<Option<Cc<Node>>> },
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
    c.bench_function("rust-cc", |b| {
        b.iter_with_large_drop(|| large_linked_list(black_box(4096)))
    });
}
