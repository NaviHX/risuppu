use std::iter;

use risuppu::{
    semantic::Env,
    sexp::{Ptr, Sexp},
};

pub fn r#do(args: Ptr<Sexp>, _env: &mut Env) -> Ptr<Sexp> {
    let mut forms = args;
    let mut v = vec![];

    for form in iter::from_fn(|| {
        if forms.is_nil() {
            None
        } else {
            let car = forms.car();
            forms = forms.cdr();
            Some(car)
        }
    }) {
        let first_form = form.car();
        if let Sexp::Identifier(ident) = first_form.as_ref() {
            if ident == "->" {
                let body = form.cdr().car();
                let params = form.cdr().cdr().car();
                v.push((body, Some(params)));

                continue;
            }
        }

        v.push((form, None));
        break;
    }

    v.into_iter()
        .rev()
        .fold(Sexp::nil(), |form, (mut body, lambda_params)| {
            if let Some(lambda_params) = lambda_params {
                let lambda = Sexp::from_vec([Sexp::lambda(), lambda_params, form]);
                let mut body: Vec<_> = iter::from_fn(|| {
                    if body.is_nil() {
                        None
                    } else {
                        let car = body.car();
                        body = body.cdr();
                        Some(car)
                    }
                })
                .collect();
                body.push(lambda);
                Sexp::from_vec(body)
            } else {
                body
            }
        })
}

#[cfg(test)]
mod test {
    use risuppu::semantic::Env;

    #[test]
    fn expand_do() {
        let mut env = Env::new();

        let expr =
            risuppu::sexp::parse::parse_sexp("(do (-> (read) (name)) (-> (print name)) name)")
                .unwrap()
                .1;
        let expanded_expr = super::r#do(expr.cdr(), &mut env);

        // let expected = Sexp::from_vec([
        //     Sexp::read(),
        //     Sexp::from_vec([
        //         Sexp::lambda(),
        //         Sexp::from_vec([Sexp::identifier("name")]),
        //         Sexp::from_vec([
        //             Sexp::print(),
        //             Sexp::identifier("name"),
        //             Sexp::from_vec([Sexp::lambda(), Sexp::nil(), Sexp::identifier("name")]),
        //         ]),
        //     ]),
        // ]);
        let expected = risuppu::sexp::parse::parse_sexp(
            "(read (lambda (name) (print name (lambda () name))))",
        )
        .unwrap()
        .1;
        assert_eq!(expanded_expr, expected);
    }
}
