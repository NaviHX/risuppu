use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::anychar;
use nom::combinator::{eof, map, peek};
use nom::multi::{fold_many0, many_till};
use nom::sequence::{delimited, preceded, tuple};
use nom::{IResult, Parser};

use crate::sexp::{Ptr, Sexp};

macro_rules! parse_sexp_keyword {
    ($s:expr, $res:expr) => {
        map(
            delimited(discard_seperator_many0, tag($s), object_tail),
            |_| $res,
        )
    };
}

macro_rules! wrap_seperator {
    ($e:expr) => {
        delimited(discard_seperator_many0, $e, object_tail)
    };
}

pub fn parse_sexp(input: &str) -> IResult<&str, Ptr<Sexp>> {
    object(input)
}

fn list(input: &str) -> IResult<&str, Ptr<Sexp>> {
    let (remaining, (object_vec, _)) = many_till(object, peek(tag(")")))(input)?;
    Ok((
        remaining,
        object_vec
            .into_iter()
            .rev()
            .fold(Sexp::nil(), |cur, obj| Sexp::cons(obj, cur)),
    ))
}

fn object(input: &str) -> IResult<&str, Ptr<Sexp>> {
    let right_paren1 = map(tag(")"), |_| ());
    let right_paren2 = map(tag(")"), |_| ());

    alt((
        wrap_seperator!(delimited(tag("("), list, right_paren1)),
        wrap_seperator!(map(delimited(tag("'("), list, right_paren2), |list| {
            Sexp::cons(Sexp::wrap(Sexp::Quote), Sexp::cons(list, Sexp::nil()))
        })),
        atom,
    ))(input)
}

fn comment(input: &str) -> IResult<&str, ()> {
    map(tuple((tag(";;"), many_till(anychar, tag("\n")))), |_| ())(input)
}

fn seperator(input: &str) -> IResult<&str, ()> {
    (map(alt((tag(" "), tag("\t"), tag("\n"))), |_| ()).or(comment)).parse(input)
}

fn discard_seperator_many0(input: &str) -> IResult<&str, ()> {
    (fold_many0(seperator, || (), |_, _| ()))(input)
}

fn discard_seperator_1(input: &str) -> IResult<&str, ()> {
    map(seperator, |_| ())(input)
}

fn object_tail(input: &str) -> IResult<&str, ()> {
    discard_seperator_many0(input)
}

fn atom(input: &str) -> IResult<&str, Ptr<Sexp>> {
    map(
        alt((
            parse_sexp_keyword!("read", Sexp::Read),
            parse_sexp_keyword!("print", Sexp::Print),
            parse_sexp_keyword!("if", Sexp::If),
            parse_sexp_keyword!("eq", Sexp::Eq),
            parse_sexp_keyword!("quote", Sexp::Quote),
            parse_sexp_keyword!("cons", Sexp::Cons),
            parse_sexp_keyword!("car", Sexp::Car),
            parse_sexp_keyword!("cdr", Sexp::Cdr),
            parse_sexp_keyword!("lambda", Sexp::Lambda),
            parse_sexp_keyword!("macro", Sexp::Macro),
            parse_sexp_keyword!("eval", Sexp::Eval),
            parse_sexp_keyword!("define", Sexp::Define),
            parse_sexp_keyword!("provide", Sexp::Provide),
            parse_sexp_keyword!("require", Sexp::Require),
            wrap_seperator!(map(preceded(tag("#\\"), anychar), Sexp::Char)),
            wrap_seperator!(map(
                preceded(tag("#"), alt((tag("t"), tag("f")))),
                |s| match s {
                    "t" => Sexp::Bool(true),
                    "f" => Sexp::Bool(false),
                    _ => Sexp::Nil,
                }
            )),
            wrap_seperator!(map(nom::character::complete::i32, Sexp::I32)),
            wrap_seperator!(sstring),
            wrap_seperator!(identifier),
        )),
        Sexp::wrap,
    )(input)
}

fn identifier(input: &str) -> IResult<&str, Sexp> {
    let right_paren = map(tag(")"), |_| ());
    let eof = map(eof, |_| ());

    map(
        many_till(anychar, peek(alt((eof, right_paren, discard_seperator_1)))),
        |(res, _)| Sexp::Identifier(res.into_iter().collect()),
    )(input)
}

fn sstring(input: &str) -> IResult<&str, Sexp> {
    map(
        delimited(tag("\""), many_till(anychar, peek(tag("\""))), tag("\"")),
        |(res, _)| {
            let s: String = res.into_iter().collect();
            let s = unescaper::unescape(&s).unwrap_or("ERROR when unescaping".to_string());
            Sexp::SString(s)
        },
    )(input)
}

#[cfg(test)]
mod test {
    use super::parse_sexp;
    use crate::sexp::Sexp;

    #[test]
    fn parse_keyword() {
        assert_eq!(parse_sexp("read").unwrap().1, Sexp::wrap(Sexp::Read));
        assert_eq!(parse_sexp("print").unwrap().1, Sexp::wrap(Sexp::Print));
        assert_eq!(parse_sexp("if").unwrap().1, Sexp::wrap(Sexp::If));
        assert_eq!(parse_sexp("eq").unwrap().1, Sexp::wrap(Sexp::Eq));
        assert_eq!(parse_sexp("quote").unwrap().1, Sexp::wrap(Sexp::Quote));
        assert_eq!(parse_sexp("cons").unwrap().1, Sexp::wrap(Sexp::Cons));
        assert_eq!(parse_sexp("car").unwrap().1, Sexp::wrap(Sexp::Car));
        assert_eq!(parse_sexp("cdr").unwrap().1, Sexp::wrap(Sexp::Cdr));
        assert_eq!(parse_sexp("lambda").unwrap().1, Sexp::wrap(Sexp::Lambda));
        assert_eq!(parse_sexp("macro").unwrap().1, Sexp::wrap(Sexp::Macro));
        assert_eq!(parse_sexp("eval").unwrap().1, Sexp::wrap(Sexp::Eval));
        assert_eq!(parse_sexp("define").unwrap().1, Sexp::wrap(Sexp::Define));
        assert_eq!(parse_sexp("provide").unwrap().1, Sexp::wrap(Sexp::Provide));
        assert_eq!(parse_sexp("require").unwrap().1, Sexp::wrap(Sexp::Require));
    }

    #[test]
    fn parse_list() {
        assert_eq!(
            parse_sexp("(read)").unwrap().1,
            Sexp::cons(Sexp::wrap(Sexp::Read), Sexp::nil())
        );
        assert_eq!(
            parse_sexp("(read read)").unwrap().1,
            Sexp::cons(
                Sexp::wrap(Sexp::Read),
                Sexp::cons(Sexp::wrap(Sexp::Read), Sexp::nil())
            )
        );
        assert_eq!(
            parse_sexp("(read read read)").unwrap().1,
            Sexp::cons(
                Sexp::wrap(Sexp::Read),
                Sexp::cons(
                    Sexp::wrap(Sexp::Read),
                    Sexp::cons(Sexp::wrap(Sexp::Read), Sexp::nil())
                )
            )
        );
    }

    #[test]
    fn parse_ident() {
        assert_eq!(
            parse_sexp("(this-is-a-ident)").unwrap().1,
            Sexp::cons(
                Sexp::wrap(Sexp::Identifier("this-is-a-ident".to_string())),
                Sexp::nil()
            )
        );
        assert_eq!(
            parse_sexp("(ident ident ident)").unwrap().1,
            Sexp::cons(
                Sexp::wrap(Sexp::Identifier("ident".to_string())),
                Sexp::cons(
                    Sexp::wrap(Sexp::Identifier("ident".to_string())),
                    Sexp::cons(
                        Sexp::wrap(Sexp::Identifier("ident".to_string())),
                        Sexp::nil()
                    )
                )
            )
        );
        assert_eq!(
            parse_sexp("ident").unwrap().1,
            Sexp::wrap(Sexp::Identifier("ident".to_string()))
        );
    }

    #[test]
    fn parse_sstring() {
        assert_eq!(
            parse_sexp("(\"test\")").unwrap().1,
            Sexp::cons(Sexp::wrap(Sexp::SString("test".to_string())), Sexp::nil())
        );
        assert_eq!(
            parse_sexp("(\"test\" \"test\" \"test\")").unwrap().1,
            Sexp::cons(
                Sexp::wrap(Sexp::SString("test".to_string())),
                Sexp::cons(
                    Sexp::wrap(Sexp::SString("test".to_string())),
                    Sexp::cons(Sexp::wrap(Sexp::SString("test".to_string())), Sexp::nil())
                )
            )
        );
        assert_eq!(
            parse_sexp("\"test\"").unwrap().1,
            Sexp::wrap(Sexp::SString("test".to_string()))
        );
    }

    #[test]
    fn parse_nested() {
        assert_eq!(
            parse_sexp("(lambda (a) (a))").unwrap().1,
            Sexp::cons(
                Sexp::wrap(Sexp::Lambda),
                Sexp::cons(
                    Sexp::cons(Sexp::wrap(Sexp::Identifier("a".to_string())), Sexp::nil()),
                    Sexp::cons(
                        Sexp::cons(Sexp::wrap(Sexp::Identifier("a".to_string())), Sexp::nil()),
                        Sexp::nil()
                    )
                )
            )
        )
    }

    #[test]
    fn parse_trailing_comment() {
        let expr = parse_sexp("(test ;; This is comment\n test)").unwrap().1;
        let expected = Sexp::from_vec([Sexp::identifier("test"), Sexp::identifier("test")]);
        assert_eq!(expr, expected);
    }

    #[test]
    fn parse_line_comment() {
        let expr = parse_sexp(";; This is comment\n(test test)").unwrap().1;
        let expected = Sexp::from_vec([Sexp::identifier("test"), Sexp::identifier("test")]);
        assert_eq!(expr, expected);
    }
}
