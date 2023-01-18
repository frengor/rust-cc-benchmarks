use std::cell::RefCell;
use cgc_single_threaded::api::*;

pub struct CgcRefCell<T>(pub RefCell<T>);

impl<T: Traceable> CgcRefCell<T> {
    pub fn new(t: T) -> CgcRefCell<T> {
        CgcRefCell(RefCell::new(t))
    }
}

impl<T: Traceable> Traceable for CgcRefCell<T> {
    fn trace_with(&self, tracer: &mut Tracer) {
        self.0.borrow().trace_with(tracer);
    }
}

impl<T> Finalizer for CgcRefCell<T> {}
