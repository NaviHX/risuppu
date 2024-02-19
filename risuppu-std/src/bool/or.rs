use risuppu::{sexp::{Ptr, Sexp}, semantic::Env};

pub fn or(args: Ptr<Sexp>, _env: &mut Env) -> Ptr<Sexp> {
    Sexp::bool(Sexp::iter(args).any(|arg| matches!(arg.as_ref(), Sexp::Bool(true))))
}

#[cfg(test)]
mod test {
    use risuppu::{semantic::Env, sexp::{parse::parse_sexp, Sexp}};

    use crate::bool::load_bool;

    #[test]
    fn or() {
        let mut env = Env::new();
        load_bool(&mut env);

        let expr = parse_sexp("(or (or #f #f) #t #f)").unwrap().1;
        let eval = env.evaluate(expr);
        assert_eq!(eval, Sexp::bool(true));

        let expr = parse_sexp("(or (or #f #f) #f #f)").unwrap().1;
        let eval = env.evaluate(expr);
        assert_eq!(eval, Sexp::bool(false));
    }
}
