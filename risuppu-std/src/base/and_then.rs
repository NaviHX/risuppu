use risuppu::{
    semantic::Env,
    sexp::{Ptr, Sexp},
};

pub fn eval_cond(args: Ptr<Sexp>, env: &mut Env) -> Ptr<Sexp> {
    let cond = args.car();
    let cont = args.cdr();

    let eval_cond = env.evaluate(cond);
    Sexp::cons(eval_cond, cont)
}

pub fn and_then(args: Ptr<Sexp>, _env: &mut Env) -> Ptr<Sexp> {
    let e = args.car();
    let c = args.cdr().car();

    if !e.is_nil() {
        Sexp::from_vec([c, e])
    } else {
        e
    }
}

#[cfg(test)]
mod test {
    use risuppu::{semantic::Env, sexp::{Sexp, parse::parse_sexp}};

    use crate::base::load_base;

    #[test]
    fn something_and_then() {
        let mut env = Env::new();
        let expr = Sexp::from_vec([Sexp::eq(), Sexp::int(1), Sexp::int(1)]);
        let cont = Sexp::from_vec([
            Sexp::lambda(),
            Sexp::from_vec([Sexp::identifier("b")]),
            Sexp::identifier("b"),
        ]);

        let expanded = super::and_then(Sexp::from_vec([expr, cont]), &mut env);
        let evaluated = env.evaluate(expanded);
        assert_eq!(evaluated, Sexp::bool(true));
    }

    #[test]
    fn none_and_then() {
        let mut env = Env::new();
        let expr = Sexp::nil();
        let cont = Sexp::from_vec([
            Sexp::lambda(),
            Sexp::from_vec([Sexp::identifier("_")]),
            Sexp::int(1),
        ]);

        let expanded = super::and_then(Sexp::from_vec([expr, cont]), &mut env);
        let evaluated = env.evaluate(expanded);
        assert_eq!(evaluated, Sexp::nil());
    }

    #[test]
    fn embedded_and_then() {
        let mut env = Env::new();
        load_base(&mut env);
        let expr = parse_sexp("(and-then (and-then (eq 1 1) (lambda (_) 1)) (lambda (_) #t))").unwrap().1;
        let eval = env.evaluate(expr);
        assert_eq!(eval, Sexp::bool(true));
    }
}
