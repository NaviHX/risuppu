use risuppu::{
    semantic::Env,
    sexp::{Ptr, Sexp},
};

pub fn modular(args: Ptr<Sexp>, env: &mut Env) -> Ptr<Sexp> {
    let init = env.evaluate(args.car());
    let ms = args.cdr();

    if let Sexp::I32(init) = init.as_ref() {
        let ans = Sexp::iter(ms)
            .filter_map(|a| match a.as_ref() {
                Sexp::I32(a) => Some(*a),
                _ => None,
            })
            .fold(*init, |pre, n| pre % n);

        Sexp::int(ans)
    } else {
        Sexp::nil()
    }
}

#[cfg(test)]
mod test {
    use risuppu::{semantic::Env, sexp::Sexp};

    #[test]
    fn modular() {
        let numbers = Sexp::from_vec([Sexp::int(6), Sexp::int(2)]);
        let mut env = Env::new();
        let sum = super::modular(numbers, &mut env);
        assert_eq!(sum, Sexp::int(0));
    }
}
