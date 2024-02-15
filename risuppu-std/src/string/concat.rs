use risuppu::sexp::{Ptr, Sexp};

pub fn concat(mut args: Ptr<Sexp>) -> Ptr<Sexp> {
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
    use risuppu::sexp::Sexp;

    #[test]
    fn concat() {
        let numbers = Sexp::from_vec([Sexp::string("One"), Sexp::string("Two"), Sexp::string("Three")]);
        let numbers = super::concat(numbers);
        assert_eq!(numbers, Sexp::string("OneTwoThree"));
    }
}
