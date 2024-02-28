use risuppu::{
    semantic::Env,
    sexp::{pattern::Pattern, Ptr, Sexp},
};

pub fn r#match(args: Ptr<Sexp>, _env: &mut Env) -> Ptr<Sexp> {
    let arg = args.car();
    let arms = args.cdr();
    let else_flag = Sexp::identifier("else");

    for arm in Sexp::iter(arms) {
        let (pat, ret_val): (Ptr<Sexp>, Ptr<Sexp>) = (arm.car(), arm.cdr().car());
        if pat == else_flag {
            return ret_val;
        }

        let pat: Pattern = pat.into();

        if let Ok(bindings) = pat.bind(arg.clone()) {
            let (params, args): (Vec<_>, Vec<_>) = bindings
                .get_binding()
                .into_iter()
                .map(|(s, v)| (Sexp::identifier(s), v))
                .unzip();
            let lambda = Sexp::from_vec([Sexp::lambda(), Sexp::from_vec(params), ret_val]);
            return Sexp::cons(lambda, Sexp::from_vec(args));
        }
    }

    Sexp::nil()
}

#[cfg(test)]
mod test {
    use risuppu::{sexp::{parse::parse_sexp, Sexp}, semantic::Env};

    use super::r#match;

    #[test]
    fn bind_name() {
        let pat = parse_sexp("'(a)").unwrap().1;
        let arg = parse_sexp("'(1)").unwrap().1;
        let ret_val = parse_sexp("a").unwrap().1;
        let expr = Sexp::from_vec([arg, Sexp::from_vec([pat, ret_val])]);

        let mut env = Env::new();
        let res = r#match(expr, &mut env);
        let res = env.evaluate(res);
        let expected = Sexp::int(1);
        assert_eq!(res, expected);
    }

    #[test]
    fn else_arm() {
        let pat = parse_sexp("'(a)").unwrap().1;
        let arg = parse_sexp("'(1 2)").unwrap().1;
        let ret_val = parse_sexp("a").unwrap().1;
        let else_flag = Sexp::identifier("else");
        let else_value = Sexp::int(3);
        let expr = Sexp::from_vec([arg, Sexp::from_vec([pat, ret_val]), Sexp::from_vec([else_flag, else_value])]);

        let mut env = Env::new();
        let res = r#match(expr, &mut env);
        let res = env.evaluate(res);
        let expected = Sexp::int(3);
        assert_eq!(res, expected);
    }
}
