use risuppu::{
    semantic::Env,
    sexp::{Ptr, Sexp},
};

use super::quote;

pub fn fold(args: Ptr<Sexp>, env: &mut Env) -> Ptr<Sexp> {
    let (lambda, init, lists) = (args.car(), args.cdr().car(), args.cdr().cdr());
    let lambda = env.evaluate(lambda);
    let mut init = env.evaluate(init);
    let lists: Vec<_> = Sexp::iter(lists).map(|list| env.evaluate(list)).collect();

    for list in lists {
        match list.as_ref() {
            Sexp::Form(_) => {
                for elem in Sexp::iter(list) {
                    init = env.evaluate(Sexp::from_vec([lambda.clone(), quote(init), elem]));
                }
            }
            _ => init = env.evaluate(Sexp::from_vec([lambda.clone(), quote(init), list])),
        }
    }

    quote(init)
}

#[cfg(test)]
mod test {
    use risuppu::{
        semantic::Env,
        sexp::{parse::parse_sexp, Sexp},
    };

    use crate::{arithmetic::load_arithmetic, list::load_list};

    #[test]
    fn sum() {
        let mut env = Env::new();
        load_list(&mut env);
        load_arithmetic(&mut env);

        let expr = parse_sexp("(__builtin_fold (lambda (a b) (__builtin_+ a b)) 0 '(1 2 3))")
            .unwrap()
            .1;
        let res = env.evaluate(expr);
        assert_eq!(res, Sexp::int(6));
    }

    #[test]
    fn sum_many_lists() {
        let mut env = Env::new();
        load_list(&mut env);
        load_arithmetic(&mut env);

        let expr = parse_sexp("(__builtin_fold (lambda (a b) (__builtin_+ a b)) 0 '(1 2 3) '(4 5 6) '(7 8 9))")
            .unwrap()
            .1;
        let res = env.evaluate(expr);
        assert_eq!(res, Sexp::int(45));
    }

    #[test]
    fn sum_atom() {
        let mut env = Env::new();
        load_list(&mut env);
        load_arithmetic(&mut env);

        let expr = parse_sexp("(__builtin_fold (lambda (a b) (__builtin_+ a b)) 0 1 2 3 4 5 6 '(7 8 9))")
            .unwrap()
            .1;
        let res = env.evaluate(expr);
        assert_eq!(res, Sexp::int(45));
    }

    #[test]
    fn rev() {
        let mut env = Env::new();
        load_list(&mut env);
        load_arithmetic(&mut env);

        let expr = parse_sexp("(__builtin_fold (lambda (a b) (cons b a)) '() '(1 2 3))")
            .unwrap()
            .1;
        let res = env.evaluate(expr);
        assert_eq!(res, Sexp::from_vec([Sexp::int(3), Sexp::int(2), Sexp::int(1)]));
    }
}
