use risuppu::{
    semantic::Env,
    sexp::{Ptr, Sexp},
};

pub fn greater(args: Ptr<Sexp>, env: &mut Env) -> Ptr<Sexp> {
    let init = env.evaluate(args.car());
    let ms = args.cdr();

    if let Sexp::I32(init) = init.as_ref() {
        let ans = Sexp::iter(ms)
            .map(|a| match a.as_ref() {
                Sexp::I32(a) => *a,
                _ => 1,
            })
            .all(|n| *init > n);

        Sexp::bool(ans)
    } else {
        Sexp::bool(false)
    }
}

pub fn less(args: Ptr<Sexp>, env: &mut Env) -> Ptr<Sexp> {
    let init = env.evaluate(args.car());
    let ms = args.cdr();

    if let Sexp::I32(init) = init.as_ref() {
        let ans = Sexp::iter(ms)
            .map(|a| match a.as_ref() {
                Sexp::I32(a) => *a,
                _ => 1,
            })
            .all(|n| *init < n);

        Sexp::bool(ans)
    } else {
        Sexp::bool(false)
    }
}

pub fn ge(args: Ptr<Sexp>, env: &mut Env) -> Ptr<Sexp> {
    let init = env.evaluate(args.car());
    let ms = args.cdr();

    if let Sexp::I32(init) = init.as_ref() {
        let ans = Sexp::iter(ms)
            .map(|a| match a.as_ref() {
                Sexp::I32(a) => *a,
                _ => 1,
            })
            .all(|n| *init >= n);

        Sexp::bool(ans)
    } else {
        Sexp::bool(false)
    }
}

pub fn le(args: Ptr<Sexp>, env: &mut Env) -> Ptr<Sexp> {
    let init = env.evaluate(args.car());
    let ms = args.cdr();

    if let Sexp::I32(init) = init.as_ref() {
        let ans = Sexp::iter(ms)
            .map(|a| match a.as_ref() {
                Sexp::I32(a) => *a,
                _ => 1,
            })
            .all(|n| *init <= n);

        Sexp::bool(ans)
    } else {
        Sexp::bool(false)
    }
}

// TODO: Test
