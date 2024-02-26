use risuppu::{
    semantic::Env,
    sexp::{Ptr, Sexp},
};

use super::quote;

pub fn flat_map(args: Ptr<Sexp>, env: &mut Env) -> Ptr<Sexp> {
    let (list, lambda) = (args.car(), args.cdr().car());
    let list = env.evaluate(list);
    let lambda = env.evaluate(lambda);

    quote(Sexp::from_vec(
        Sexp::iter(list)
            .flat_map(|elem| Sexp::iter(env.evaluate(Sexp::from_vec([lambda.clone(), elem]))))
            .collect::<Vec<_>>(),
    ))
}

#[cfg(test)]
mod test {
    use risuppu::{semantic::Env, sexp::parse::parse_sexp};

    use crate::{list::load_list, arithmetic::load_arithmetic};

    #[test]
    fn flat_map() {
        let mut env = Env::new();
        load_list(&mut env);
        load_arithmetic(&mut env);

        let expr = parse_sexp("(__builtin_flat-map '(1 2 3) (lambda (a) (__builtin_list a (__builtin_* a 2) (__builtin_* a 3))))")
            .unwrap()
            .1;
        let res = env.evaluate(expr);
        let expected = parse_sexp("(1 2 3 2 4 6 3 6 9)").unwrap().1;
        assert_eq!(res, expected);
    }

    #[test]
    fn filter() {
        let mut env = Env::new();
        load_list(&mut env);
        load_arithmetic(&mut env);

        let expr = parse_sexp("(__builtin_flat-map '(1 2 3) (lambda (a) (if (eq a 2) (__builtin_list a) '())))")
            .unwrap()
            .1;
        let res = env.evaluate(expr);
        let expected = parse_sexp("(2)").unwrap().1;
        assert_eq!(res, expected);
    }
}
