use risuppu::{
    semantic::Env,
    sexp::{Ptr, Sexp},
};

fn omega() -> Ptr<Sexp> {
    Sexp::from_vec([
        Sexp::lambda(),
        Sexp::from_vec([Sexp::identifier("f")]),
        Sexp::from_vec([Sexp::identifier("f"), Sexp::identifier("f")]),
    ])
}

fn y(params: Vec<Ptr<Sexp>>) -> Ptr<Sexp> {
    let params = Sexp::from_vec(params);
    Sexp::from_vec([
        Sexp::lambda(),
        Sexp::from_vec([Sexp::identifier("f")]),
        Sexp::from_vec([
            omega(),
            Sexp::from_vec([
                Sexp::lambda(),
                Sexp::from_vec([Sexp::identifier("g")]),
                Sexp::from_vec([
                    Sexp::lambda(),
                    params.clone(),
                    Sexp::cons(
                        Sexp::from_vec([
                            Sexp::identifier("f"),
                            Sexp::from_vec([Sexp::identifier("g"), Sexp::identifier("g")]),
                        ]),
                        params,
                    ),
                ]),
            ]),
        ]),
    ])
}

pub fn r#let(args: Ptr<Sexp>, _env: &mut Env) -> Ptr<Sexp> {
    let first_form = args.car();
    let (named, decls, cont) = if let Sexp::Identifier(_) = first_form.as_ref() {
        (Some(first_form), args.cdr().car(), args.cdr().cdr().car())
    } else {
        (None, first_form, args.cdr().car())
    };

    let (idents, decls): (Vec<_>, Vec<_>) = Sexp::iter(decls)
        .filter_map(|decl| {
            let ident = decl.car();
            let decl = decl.cdr().car();

            if let Sexp::Identifier(_) = ident.as_ref() {
                Some((ident, decl))
            } else {
                None
            }
        })
        .unzip();

    let lambda = Sexp::from_vec([Sexp::lambda(), Sexp::from_vec(idents), cont]);
    if let Some(named) = named {
        let named_lambda = Sexp::from_vec([
            Sexp::lambda(),
            Sexp::from_vec([named]),
            Sexp::cons(lambda.clone(), Sexp::from_vec(decls)),
        ]);
        Sexp::from_vec([named_lambda, lambda])
    } else {
        Sexp::cons(lambda, Sexp::from_vec(decls))
    }
}

#[cfg(test)]
mod test {
    use risuppu::{semantic::Env, sexp::Sexp};

    use crate::{arithmetic::load_arithmetic, base::load_base};

    #[test]
    fn let_1() {
        let mut env = Env::new();
        let expr = risuppu::sexp::parse::parse_sexp("(((a 1)) (eq a 1))")
            .unwrap()
            .1;
        let expanded = super::r#let(expr, &mut env);
        let expected = risuppu::sexp::parse::parse_sexp("((lambda (a) (eq a 1)) 1)")
            .unwrap()
            .1;
        assert_eq!(expanded, expected);
    }

    #[test]
    fn let_many() {
        let mut env = Env::new();
        let expr = risuppu::sexp::parse::parse_sexp("(((a 1) (b 2)) (eq a b))")
            .unwrap()
            .1;
        let expanded = super::r#let(expr, &mut env);
        let expected = risuppu::sexp::parse::parse_sexp("((lambda (a b) (eq a b)) 1 2)")
            .unwrap()
            .1;
        assert_eq!(expanded, expected);
    }

    #[test]
    fn let_named() {
        let mut env = Env::new();
        let expr = risuppu::sexp::parse::parse_sexp("(loop ((a 1) (b 2)) (loop a b))")
            .unwrap()
            .1;
        let expanded = super::r#let(expr, &mut env);
        let expected = risuppu::sexp::parse::parse_sexp(
            "((lambda (loop) ((lambda (a b) (loop a b)) 1 2)) (lambda (a b) (loop a b)))",
        )
        .unwrap()
        .1;
        assert_eq!(expanded, expected);
    }

    #[test]
    fn let_list() {
        let mut env = Env::new();
        let expr = risuppu::sexp::parse::parse_sexp("(let ((l '(1 2))) (car l))")
            .unwrap()
            .1;
        let expanded = super::r#let(expr, &mut env);
        let expected = Sexp::int(1);
        assert_eq!(env.evaluate(expanded), expected);
    }

    #[test]
    fn let_loop() {
        let mut env = Env::new();
        load_base(&mut env);
        load_arithmetic(&mut env);
        let expr = risuppu::sexp::parse::parse_sexp(
            "(let loop ((n 5)) (+ n (if (eq n 1) 0 (loop (- n 1)))))",
        )
        .unwrap()
        .1;
        let evaluated = env.evaluate(expr);
        let expected = Sexp::int(15);
        assert_eq!(evaluated, expected);
    }
}
