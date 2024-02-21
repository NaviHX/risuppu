use risuppu::{sexp::{Ptr, Sexp}, semantic::Env};

pub fn seq(mut args: Ptr<Sexp>, _env: &mut Env) -> Ptr<Sexp> {
    let mut ret = Sexp::nil();

    while !args.is_nil() {
        let expr = args.car();
        args = args.cdr();

        // ret = env.evaluate(expr);
        ret = expr;
    }

    ret
}

#[cfg(test)]
mod test {
    use risuppu::{sexp::{Sexp, parse::parse_sexp}, semantic::Env};

    use crate::base::load_base;

    #[test]
    fn seq() {
        let form = Sexp::from_vec([
            Sexp::identifier("seq"),
            Sexp::from_vec([Sexp::define(), Sexp::identifier("a"), Sexp::int(1)]),
            Sexp::from_vec([Sexp::eq(), Sexp::int(1), Sexp::int(2)]),
        ]);
        let mut env = Env::new();
        load_base(&mut env);

        let ans = env.evaluate(form);
        assert_eq!(ans, Sexp::bool(false));
        assert_eq!(env.get("a"), Some(Sexp::int(1)));
    }

    #[test]
    fn protect_stack() {
        let mut env = Env::new();
        load_base(&mut env);
        let expr = Sexp::from_vec([
            Sexp::identifier("seq"),
            parse_sexp("(define a 1)").unwrap().1,
            parse_sexp("(define b a)").unwrap().1,
            parse_sexp("((lambda (a b) ()) 2)").unwrap().1,
            parse_sexp("(define b a)").unwrap().1,
            parse_sexp("b").unwrap().1,
        ]);
        let res = env.evaluate(expr);
        assert_eq!(res, Sexp::int(1));
    }
}
