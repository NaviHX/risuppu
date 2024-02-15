use std::iter;

use risuppu::sexp::{Ptr, Sexp};

pub fn plus(mut args: Ptr<Sexp>) -> Ptr<Sexp> {
    let sum = iter::from_fn(|| {
        if args.is_nil() {
            None
        } else {
            let car = args.car();
            args = args.cdr();
            Some(car)
        }
    })
    .map(|a| match a.as_ref() {
        Sexp::I32(a) => *a,
        _ => 0,
    })
    .sum();

    Sexp::int(sum)
}

#[cfg(test)]
mod test {
    use risuppu::sexp::Sexp;

    #[test]
    fn plus() {
        let numbers = Sexp::from_vec([Sexp::int(1), Sexp::int(2), Sexp::int(3)]);
        let sum = super::plus(numbers);
        assert_eq!(sum, Sexp::int(6));
    }

    #[test]
    fn plus_0() {
        let list = super::plus(Sexp::nil());
        let sum = super::plus(list);
        assert_eq!(sum, Sexp::int(0));
    }
}
