use std::cell::RefCell;
use std::hint::black_box;
use std::rc::{Rc, Weak};

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
    head: Rc<Node>,
}

impl List {
    fn new() -> List {
        List {
            head: Rc::new(Node::Nil),
        }
    }

    fn add(&mut self) {
        let cons = Rc::new(Node::Cons {
            next: self.head.clone(),
            previous: RefCell::new(None),
        });
        if let Node::Cons{ previous, .. } = &*self.head {
            *previous.borrow_mut() = Some(Rc::downgrade(&cons));
        }
        self.head = cons;
    }

    fn len(&self) -> usize {
        self.head.len()
    }
}

enum Node {
    Cons { next: Rc<Node>, previous: RefCell<Option<Weak<Node>>> },
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
    c.bench_function("rc", |b| {
        b.iter(|| large_linked_list(black_box(4096)))
    });
}
