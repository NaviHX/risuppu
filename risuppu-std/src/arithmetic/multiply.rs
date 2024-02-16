use std::iter;

use risuppu::{
    semantic::Env,
    sexp::{Ptr, Sexp},
};

pub fn multiply(mut args: Ptr<Sexp>, env: &mut Env) -> Ptr<Sexp> {
    let sum = iter::from_fn(|| {
        if args.is_nil() {
            None
        } else {
            let car = env.evaluate(args.car());
            args = args.cdr();
            Some(car)
        }
    })
    .map(|a| match a.as_ref() {
        Sexp::I32(a) => *a,
        _ => 1,
    })
    .product();

    Sexp::int(sum)
}

#[cfg(test)]
mod test {
    use risuppu::{semantic::Env, sexp::Sexp};

    #[test]
    fn multiply() {
        let numbers = Sexp::from_vec([Sexp::int(1), Sexp::int(2), Sexp::int(3)]);
        let mut env = Env::new();
        let sum = super::multiply(numbers, &mut env);
        assert_eq!(sum, Sexp::int(6));
    }

    #[test]
    fn multiply_0() {
        let list = Sexp::from_vec([Sexp::nil()]);
        let mut env = Env::new();
        let sum = super::multiply(list, &mut env);
        assert_eq!(sum, Sexp::int(1));
    }
}
