use std::iter;

use risuppu::{
    semantic::Env,
    sexp::{Ptr, Sexp},
};

pub fn minus(args: Ptr<Sexp>, _env: &mut Env) -> Ptr<Sexp> {
    let init = args.car();
    let mut ms = args.cdr();

    if let Sexp::I32(init) = init.as_ref() {
        let ans = iter::from_fn(|| {
            if ms.is_nil() {
                None
            } else {
                let car = ms.car();
                ms = ms.cdr();
                Some(car)
            }
        })
        .map(|a| match a.as_ref() {
            Sexp::I32(a) => *a,
            _ => 0,
        })
        .fold(*init, |pre, n| pre - n);

        Sexp::int(ans)
    } else {
        Sexp::nil()
    }
}

#[cfg(test)]
mod test {
    use risuppu::{sexp::Sexp, semantic::Env};

    #[test]
    fn minus() {
        let numbers = Sexp::from_vec([Sexp::int(1), Sexp::int(2), Sexp::int(3)]);
        let mut env = Env::new();
        let sum = super::minus(numbers, &mut env);
        assert_eq!(sum, Sexp::int(-4));
    }

    #[test]
    fn minus_0() {
        let list = Sexp::from_vec([Sexp::nil()]);
        let mut env = Env::new();
        let sum = super::minus(list, &mut env);
        assert_eq!(sum, Sexp::nil());
    }
}
