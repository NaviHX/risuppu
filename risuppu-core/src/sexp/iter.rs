use super::{Ptr, Sexp};

pub struct SexpListIter {
    head: Ptr<Sexp>,
}

impl SexpListIter {
    pub fn new(head: Ptr<Sexp>) -> Self {
        Self { head }
    }
}

impl Iterator for SexpListIter {
    type Item = Ptr<Sexp>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.head.is_nil() {
            None
        } else {
            let car = self.head.car();
            self.head = self.head.cdr();
            Some(car)
        }
    }
}
