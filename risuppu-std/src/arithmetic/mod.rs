use crate::pre_function;

mod plus;
mod minus;
mod multiply;
mod divide;
mod modular;
mod comp;

super::std_library!(
    arithmetic,
    (plus::plus, "__builtin_+", pre_function),
    (minus::minus, "__builtin_-", pre_function),
    (multiply::multiply, "__builtin_*", pre_function),
    (divide::divide, "__builtin_/", pre_function),
    (modular::modular, "__builtin_mod", pre_function),
    (comp::less, "__builtin_less", pre_function),
    (comp::greater, "__builtin_greater", pre_function),
    (comp::le, "__builtin_le", pre_function),
    (comp::ge, "__builtin_ge", pre_function)
);

#[cfg(test)]
mod test {
    use risuppu::{semantic::Env, sexp::{parse::parse_sexp, Sexp}};

    #[test]
    fn fact() {
        let mut env = Env::new();
        super::load_arithmetic(&mut env);

        env.evaluate(
            parse_sexp("(define fact (lambda (n) (if (eq n 1) 1 (__builtin_* n (fact (__builtin_- n 1))))))")
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
            parse_sexp("(__builtin_+ 1 (__builtin_+ 2 3))")
                .unwrap()
                .1,
        );

        assert_eq!(res, Sexp::int(6));
    }
}
