use risuppu::{sexp::{Ptr, Sexp}, semantic::Env};

pub fn r#let(args: Ptr<Sexp>, env: &mut Env) -> Ptr<Sexp> {
    let first_form = args.car();
    let (named, decls, cont) = if let Sexp::Identifier(_) = first_form.as_ref() {
        (Some(first_form), args.cdr().car(), args.cdr().cdr().car())
    } else {
        (None, first_form, args.cdr().car())
    };

    let (idents, decls): (Vec<_>, Vec<_>) = Sexp::iter(decls).filter_map(|decl| {
        let ident = decl.car();
        let decl = decl.cdr().car();

        if let Sexp::Identifier(_) = ident.as_ref() {
            let evaluated = env.evaluate(decl);
            Some((ident, evaluated))
        } else {
            None
        }
    }).unzip();

    if let Some(_named) = named {
        // TODO: impl named let
        todo!()
    } else {
        let lambda = Sexp::from_vec([Sexp::lambda(), Sexp::from_vec(idents), cont]);
        Sexp::cons(lambda, Sexp::from_vec(decls))
    }
}

#[cfg(test)]
mod test {
    use risuppu::semantic::Env;

    #[test]
    fn let_1() {
        let mut env = Env::new();
        let expr = risuppu::sexp::parse::parse_sexp("(((a 1)) (eq a 1))").unwrap().1;
        let expanded = super::r#let(expr, &mut env);
        let expected = risuppu::sexp::parse::parse_sexp("((lambda (a) (eq a 1)) 1)").unwrap().1;
        assert_eq!(expanded, expected);
    }

    #[test]
    fn let_many() {
        let mut env = Env::new();
        let expr = risuppu::sexp::parse::parse_sexp("(((a 1) (b 2)) (eq a b))").unwrap().1;
        let expanded = super::r#let(expr, &mut env);
        let expected = risuppu::sexp::parse::parse_sexp("((lambda (a b) (eq a b)) 1 2)").unwrap().1;
        assert_eq!(expanded, expected);
    }
}
