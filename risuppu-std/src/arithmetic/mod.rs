use crate::pre_function;

super::std_library!(
    arithmetic,
    (plus, "+", pre_function),
    (minus, "-", pre_function),
    (multiply, "*", pre_function),
    (divide, "/", pre_function)
);

#[cfg(test)]
mod test {
    use risuppu::{semantic::Env, sexp::{parse::parse_sexp, Sexp}};

    #[test]
    fn fact() {
        let mut env = Env::new();
        super::load_arithmetic(&mut env);

        env.evaluate(
            parse_sexp("(define fact (lambda (n) (if (eq n 1) 1 (* n (fact (- n 1))))))")
                .unwrap()
                .1,
        );

        assert_eq!(
            env.evaluate(parse_sexp("(fact 1)").unwrap().1),
            Sexp::int(1)
        );
        assert_eq!(
            env.evaluate(parse_sexp("(fact 2)").unwrap().1),
            Sexp::int(2)
        );
        assert_eq!(
            env.evaluate(parse_sexp("(fact 3)").unwrap().1),
            Sexp::int(6)
        );
    }

    #[test]
    fn embedded_plus() {
        let mut env = Env::new();
        super::load_arithmetic(&mut env);

        let res = env.evaluate(
            parse_sexp("(+ 1 (+ 2 3))")
                .unwrap()
                .1,
        );

        assert_eq!(res, Sexp::int(6));
    }
}
