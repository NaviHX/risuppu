#[macro_export]
macro_rules! sexp {
    (($($list:tt)*)) => {
        $crate::form!($($list)*)
    };

    ($($obj:tt)*) => {
        $crate::atom!($($obj)*)
    };
}

#[macro_export]
macro_rules! form {
    (#[$car:tt], $($cdr:tt)*) => {
        {
            let cdr = $crate::form!($($cdr)*);
            $crate::sexp::Sexp::cons($crate::sexp!(#[$car]), cdr)
        }
    };

    (#[$car:tt]) => {
        {
            let nil = $crate::sexp::Sexp::nil();
            $crate::sexp::Sexp::cons($crate::sexp!(#[$car]), nil)
        }
    };

    ($car:tt, $($cdr:tt)*) => {
        {
            let cdr = $crate::form!($($cdr)*);
            $crate::sexp::Sexp::cons($crate::sexp!($car), cdr)
        }
    };

    ($car:tt) => {
        {
            let nil = $crate::sexp::Sexp::nil();
            $crate::sexp::Sexp::cons($crate::sexp!($car), nil)
        }
    };

    (. $($cdr:tt)*) => {
        $crate::sexp!($($cdr)*)
    };

    () => {
        $crate::sexp::Sexp::nil()
    }
}

#[macro_export]
macro_rules! atom {
    (read) => {
        $crate::sexp::Sexp::read()
    };

    (print) => {
        $crate::sexp::Sexp::print()
    };

    (if) => {
        $crate::sexp::Sexp::r#if()
    };

    (eq) => {
        $crate::sexp::Sexp::eq()
    };

    (quote) => {
        $crate::sexp::Sexp::quote()
    };

    (cons) => {
        $crate::sexp::Sexp::cons()
    };

    (car) => {
        $crate::sexp::Sexp::car()
    };

    (cdr) => {
        $crate::sexp::Sexp::cdr()
    };

    (lambda) => {
        $crate::sexp::Sexp::lambda()
    };

    (macro) => {
        $crate::sexp::Sexp::macro()
    };

    (eval) => {
        $crate::sexp::Sexp::eval()
    };

    (define) => {
        $crate::sexp::Sexp::define()
    };

    (provide) => {
        $crate::sexp::Sexp::provide()
    };

    (require) => {
        $crate::sexp::Sexp::require()
    };

    (nil) => {
        $crate::sexp::Sexp::nil()
    };

    (#[$($v:tt)*]) => {
        $crate::sexp::Sexp::identifier($($v)*)
    };

    ($v:expr) => {
        $crate::sexp::Sexp::wrap($crate::sexp::Sexp::from($v))
    };
}

#[cfg(test)]
mod test {
    use crate::sexp::Sexp;

    #[test]
    fn ident_atom() {
        let ident = crate::atom!(#["ident"]);
        assert_eq!(ident, Sexp::identifier("ident"));

        let ident = crate::sexp!(#["ident"]);
        assert_eq!(ident, Sexp::identifier("ident"));
    }

    #[test]
    fn literal_atom() {
        let expr = crate::atom!(1);
        assert_eq!(expr, Sexp::int(1));

        let expr = crate::atom!('a');
        assert_eq!(expr, Sexp::char('a'));

        let expr = crate::atom!("string".to_string());
        assert_eq!(expr, Sexp::string("string"));

        let expr = crate::atom!(true);
        assert_eq!(expr, Sexp::bool(true));
    }

    #[test]
    fn list_0() {
        let expr = crate::form!();
        assert_eq!(expr, Sexp::nil());

        let expr = crate::sexp!(());
        assert_eq!(expr, Sexp::nil());
    }

    #[test]
    fn list_1() {
        let expected = Sexp::from_vec([Sexp::int(1)]);

        let expr = crate::form!(1);
        assert_eq!(expr, expected);

        let expr = crate::sexp!((1));
        assert_eq!(expr, expected);
    }

    #[test]
    fn list_many() {
        let expected = Sexp::from_vec([Sexp::define(), Sexp::identifier("a"), Sexp::int(1)]);

        let expr = crate::form!(define, #["a"], 1);
        assert_eq!(expr, expected);

        let expr = crate::sexp!((define, #["a"], 1));
        assert_eq!(expr, expected);
    }

    #[test]
    fn list_embedded() {
        let expected = crate::sexp::parse::parse_sexp("((1 2) 3)").unwrap().1;
        let expr = crate::sexp!(((1, 2), 3));
        assert_eq!(expected, expr);
    }

    #[test]
    fn list_identifier() {
        let expected = crate::sexp::parse::parse_sexp("(ident)").unwrap().1;
        let expr = crate::sexp!((#["ident"]));
        assert_eq!(expected, expr);
    }
}
