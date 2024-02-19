use risuppu::{sexp::{Ptr, Sexp}, semantic::Env};

pub fn and(args: Ptr<Sexp>, _env: &mut Env) -> Ptr<Sexp> {
    Sexp::bool(Sexp::iter(args).all(|arg| !matches!(arg.as_ref(), Sexp::Bool(false))))
}

#[cfg(test)]
mod test {
    use risuppu::{semantic::Env, sexp::{parse::parse_sexp, Sexp}};

    use crate::bool::load_bool;

    #[test]
    fn and() {
        let mut env = Env::new();
        load_bool(&mut env);

        let expr = parse_sexp("(and (and #t #t) #t #f)").unwrap().1;
        let eval = env.evaluate(expr);
        assert_eq!(eval, Sexp::bool(false));

        let expr = parse_sexp("(and (and #t #t) #t #t)").unwrap().1;
        let eval = env.evaluate(expr);
        assert_eq!(eval, Sexp::bool(true));
    }
}
