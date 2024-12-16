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

#[cfg(test)]
mod test {
    use crate::sexp::{Cons, Ptr, Sexp};

    #[test]
    fn iter_nothing() {
        let expr = Sexp::from_vec::<[Ptr<Sexp>; 0]>([]);
        let mut it = Sexp::iter(expr);

        assert_eq!(it.next(), None);
    }

    #[test]
    fn iter_nil_form() {
        let expr = Sexp::Form(Cons {
            car: Sexp::nil(),
            cdr: Sexp::nil(),
        });
        let expr = Ptr::new(expr);
        let mut it = Sexp::iter(expr);

        assert_eq!(it.next(), Some(Sexp::nil()));
        assert_eq!(it.next(), None);
    }
}
