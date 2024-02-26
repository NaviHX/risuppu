use risuppu::{
    semantic::Env,
    sexp::{Ptr, Sexp},
};

pub fn map(args: Ptr<Sexp>, env: &mut Env) -> Ptr<Sexp> {
    let (list, lambda) = (args.car(), args.cdr().car());
    let list = env.evaluate(list);
    let lambda = env.evaluate(lambda);

    Sexp::from_vec(
        Sexp::iter(list)
            .map(|elem| env.evaluate(Sexp::from_vec([lambda.clone(), elem])))
            .collect::<Vec<_>>(),
    )
}

#[cfg(test)]
mod test {
    use risuppu::{semantic::Env, sexp::{parse::parse_sexp, Sexp}};

    use crate::{list::load_list, arithmetic::load_arithmetic};

    #[test]
    fn map() {
        let mut env = Env::new();
        load_list(&mut env);
        load_arithmetic(&mut env);

        let expr = parse_sexp("(map '(1 2 3) (lambda (a) (* a 2)))")
            .unwrap()
            .1;
        let res = env.evaluate(expr);
        assert_eq!(res, Sexp::from_vec([Sexp::int(2), Sexp::int(4), Sexp::int(6)]));
    }
}
