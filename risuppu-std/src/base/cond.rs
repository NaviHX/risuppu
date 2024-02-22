use risuppu::{sexp::{Ptr, Sexp}, semantic::Env};

pub fn cond(args: Ptr<Sexp>, _env: &mut Env) -> Ptr<Sexp> {
    let mut arms = vec![];
    let mut else_arm = Sexp::nil();
    let else_flag = Sexp::identifier("else");

    for arm in Sexp::iter(args) {
        let (c, b) = (arm.car(), arm.cdr().car());
        if c == else_flag {
            else_arm = b;
            break;
        }

        arms.push((c, b));
    }

    arms.into_iter().rev().fold(else_arm, |else_branch, (condition, branch)| {
        Sexp::from_vec([Sexp::r#if(), condition, branch, else_branch])
    })
}

#[cfg(test)]
mod test {
    use risuppu::{semantic::Env, sexp::{parse::parse_sexp, Sexp}};

    use crate::base::load_base;

    #[test]
    fn expand_cond() {
        let mut env = Env::new();
        let expr = parse_sexp("(((eq n 1) 2) ((eq n 2) 1) (else 3))").unwrap().1;
        let expanded = super::cond(expr, &mut env);
        let expected = parse_sexp("(if (eq n 1) 2 (if (eq n 2) 1 3))").unwrap().1;
        assert_eq!(expanded, expected);
    }

    #[test]
    fn cond() {
        let mut env = Env::new();
        load_base(&mut env);
        let expr = parse_sexp("(cond ((eq n 1) 2) ((eq n 2) 1) (else 3))").unwrap().1;

        env.set_global("n", Sexp::int(1));
        assert_eq!(env.evaluate(expr.clone()), Sexp::int(2));

        env.set_global("n", Sexp::int(2));
        assert_eq!(env.evaluate(expr.clone()), Sexp::int(1));

        env.set_global("n", Sexp::int(3));
        assert_eq!(env.evaluate(expr.clone()), Sexp::int(3));
    }
}
