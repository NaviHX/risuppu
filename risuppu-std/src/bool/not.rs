use risuppu::{sexp::{Ptr, Sexp}, semantic::Env};

pub fn not(args: Ptr<Sexp>, _env: &mut Env) -> Ptr<Sexp> {
    let arg = args.car();
    Sexp::bool(matches!(arg.as_ref(), Sexp::Bool(false)))
}

#[cfg(test)]
mod test {
    use risuppu::{semantic::Env, sexp::{parse::parse_sexp, Sexp}};

    use crate::bool::load_bool;

    #[test]
    fn not() {
        let mut env = Env::new();
        load_bool(&mut env);

        let expr = parse_sexp("(not (not #t))").unwrap().1;
        let eval = env.evaluate(expr);
        assert_eq!(eval, Sexp::bool(true));

        let expr = parse_sexp("(not (not #f))").unwrap().1;
        let eval = env.evaluate(expr);
        assert_eq!(eval, Sexp::bool(false));

        let expr = parse_sexp("(not #t)").unwrap().1;
        let eval = env.evaluate(expr);
        assert_eq!(eval, Sexp::bool(false));

        let expr = parse_sexp("(not #f)").unwrap().1;
        let eval = env.evaluate(expr);
        assert_eq!(eval, Sexp::bool(true));
    }
}
