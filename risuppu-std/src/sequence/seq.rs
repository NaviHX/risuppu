use risuppu::{sexp::{Ptr, Sexp}, semantic::Env};

pub fn seq(mut args: Ptr<Sexp>, env: &mut Env) -> Ptr<Sexp> {
    let mut ret = Sexp::nil();

    while !args.is_nil() {
        let expr = args.car();
        args = args.cdr();

        ret = env.evaluate(expr);
    }

    ret
}

#[cfg(test)]
mod test {
    use risuppu::{sexp::Sexp, semantic::Env};

    #[test]
    fn seq() {
        let form = Sexp::from_vec([
            Sexp::from_vec([Sexp::define(), Sexp::identifier("a"), Sexp::int(1)]),
            Sexp::from_vec([Sexp::eq(), Sexp::int(1), Sexp::int(2)]),
        ]);
        let mut env = Env::new();

        let ans = super::seq(form, &mut env);
        assert_eq!(ans, Sexp::bool(false));
        assert_eq!(env.get("a"), Some(Sexp::int(1)));
    }
}
