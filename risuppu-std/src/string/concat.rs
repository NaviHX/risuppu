use risuppu::{sexp::{Ptr, Sexp}, semantic::Env};

pub fn concat(mut args: Ptr<Sexp>, _env: &mut Env) -> Ptr<Sexp> {
    let mut v = vec![];
    while !args.is_nil() {
        let arg = args.car();
        if let Sexp::SString(s) = arg.as_ref() {
            v.push(s.clone());
        } else {
            v.push(arg.to_string());
        }
        args = args.cdr();
    }

    let len = v.iter().map(|s| s.len()).sum();
    let mut buf = String::with_capacity(len);
    for s in v {
        buf.push_str(s.as_str());
    }

    Sexp::string(buf)
}

#[cfg(test)]
mod test {
    use risuppu::{sexp::Sexp, semantic::Env};

    #[test]
    fn concat() {
        let numbers = Sexp::from_vec([Sexp::string("One"), Sexp::string("Two"), Sexp::string("Three")]);
        let mut env = Env::new();
        let numbers = super::concat(numbers, &mut env);
        assert_eq!(numbers, Sexp::string("OneTwoThree"));
    }
}