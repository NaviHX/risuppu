pub mod macros;
pub mod iter;
pub mod parse;
pub mod pattern;
pub mod rustfn;
use gc::{Finalize, Gc, GcCell, Trace};
use std::fmt::Display;

use self::{iter::SexpListIter, rustfn::RustFn};
use crate::semantic::{frame::Frame, Env};

pub type Ptr<T> = Gc<T>;

#[derive(Debug, PartialEq, Eq, Trace, Finalize)]
pub enum Sexp {
    // IO
    Read,
    Print,

    // Condition
    If,
    Eq,

    // Quote
    Quote,

    // List, Tree
    Cons,
    Car,
    Cdr,

    // Lambda
    Lambda,
    CapturedLambda(Gc<GcCell<Frame>>),
    // Macro, a kind of special lambdas, with every param quoted
    Macro,
    // Rust function
    RustFn(RustFn),

    // Evaluate
    Eval,

    // Define
    Define,

    // Data
    I32(i32),
    Char(char),
    SString(String),
    Bool(bool),

    // Module support
    Provide,
    Require,

    // Identity
    Identifier(String),

    Nil,
    Form(Cons),
}

#[derive(Debug, PartialEq, Eq, Trace, Finalize)]
pub struct Cons {
    pub car: Ptr<Sexp>,
    pub cdr: Ptr<Sexp>,
}

impl Cons {
    pub fn new(car: Ptr<Sexp>, cdr: Ptr<Sexp>) -> Self {
        Self { car, cdr }
    }
}

macro_rules! keyword_wrapper {
    ($f:ident, $k:expr) => {
        pub fn $f() -> Ptr<Sexp> {
            Sexp::wrap($k)
        }
    };
}

macro_rules! literal_wrapper {
    ($f:ident, $t:ty, $wrapper:expr) => {
        pub fn $f(expr: $t) -> Ptr<Sexp> {
            Sexp::wrap($wrapper(expr))
        }
    };
}

impl Sexp {
    pub fn car(&self) -> Ptr<Self> {
        if let Self::Form(Cons { car, cdr: _ }) = self {
            car.clone()
        } else {
            Ptr::new(Sexp::Nil)
        }
    }

    pub fn cdr(&self) -> Ptr<Self> {
        if let Self::Form(Cons { car: _, cdr }) = self {
            cdr.clone()
        } else {
            Ptr::new(Sexp::Nil)
        }
    }

    pub fn is_nil(&self) -> bool {
        matches!(self, Self::Nil)
    }

    pub fn is_lambda(&self) -> bool {
        matches!(self, Self::Lambda | Self::CapturedLambda(_))
    }

    pub fn is_macro(&self) -> bool {
        matches!(self, Self::Macro)
    }

    pub fn cons(l: Ptr<Self>, r: Ptr<Self>) -> Ptr<Self> {
        Ptr::new(Sexp::Form(Cons { car: l, cdr: r }))
    }

    pub fn nil() -> Ptr<Sexp> {
        Ptr::new(Sexp::Nil)
    }

    pub fn wrap(expr: Self) -> Ptr<Self> {
        Ptr::new(expr)
    }

    pub fn from_vec<T>(v: T) -> Ptr<Self>
    where
        T: IntoIterator,
        <T as std::iter::IntoIterator>::IntoIter: std::iter::DoubleEndedIterator,
        <T as std::iter::IntoIterator>::Item: Into<Ptr<Self>>,
    {
        v.into_iter()
            .rev()
            .fold(Self::nil(), |list, cur| Self::cons(cur.into(), list))
    }

    keyword_wrapper!(read, Sexp::Read);
    keyword_wrapper!(print, Sexp::Print);
    keyword_wrapper!(r#if, Sexp::If);
    keyword_wrapper!(eq, Sexp::Eq);
    keyword_wrapper!(quote, Sexp::Quote);
    keyword_wrapper!(car_token, Sexp::Car);
    keyword_wrapper!(cdr_token, Sexp::Cdr);
    keyword_wrapper!(lambda, Sexp::Lambda);
    keyword_wrapper!(r#macro, Sexp::Macro);
    keyword_wrapper!(eval, Sexp::Eval);
    keyword_wrapper!(define, Sexp::Define);
    keyword_wrapper!(require, Sexp::Require);
    keyword_wrapper!(provide, Sexp::Provide);

    literal_wrapper!(int, i32, Sexp::I32);
    literal_wrapper!(r#char, char, Sexp::Char);
    literal_wrapper!(r#bool, bool, Sexp::Bool);

    pub fn string(s: impl ToString) -> Ptr<Self> {
        Sexp::wrap(Sexp::SString(s.to_string()))
    }

    pub fn identifier(s: impl ToString) -> Ptr<Self> {
        Sexp::wrap(Sexp::Identifier(s.to_string()))
    }

    pub fn lambda_capture(frame_ptr: Gc<GcCell<Frame>>) -> Ptr<Self> {
        Sexp::wrap(Sexp::CapturedLambda(frame_ptr))
    }

    /// # Safety
    /// Don't capture `Gc` value in the closure, which will escape from the gc management.
    /// Don't recurse in the function body.
    /// Quote the ret-value if it might be a list and this function is not a macro.
    pub unsafe fn rust_fn(f: impl FnMut(Ptr<Sexp>, &mut Env) -> Ptr<Sexp> + 'static) -> Ptr<Self> {
        Sexp::wrap(Sexp::RustFn(RustFn::new(f)))
    }

    /// # Safety
    /// Don't capture `Gc` value in the closure, which will escape from the gc management.
    /// Don't recurse in f's body.
    /// Quote the ret-value if it might be a list and this function is not a macro.
    pub unsafe fn rust_fn_with_preprocess(
        f: impl FnMut(Ptr<Sexp>, &mut Env) -> Ptr<Sexp> + 'static,
        p: impl Fn(Ptr<Sexp>, &mut Env) -> Ptr<Sexp> + 'static,
    ) -> Ptr<Self> {
        Sexp::wrap(Sexp::RustFn(RustFn::new_with_preprocess(f, p)))
    }

    pub fn iter(list: Ptr<Sexp>) -> SexpListIter {
        SexpListIter::new(list)
    }

    pub fn is_quoted(&self) -> bool {
        matches!(self.car().as_ref(), Self::Quote)
    }

    pub fn get_quoted(&self) -> Option<Ptr<Sexp>> {
        if self.is_quoted() {
            Some(self.cdr().car())
        } else {
            None
        }
    }

    pub fn is_identifier(&self) -> bool {
        matches!(self, Self::Identifier(_))
    }
}

impl Display for Sexp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sexp::Read => write!(f, "read"),
            Sexp::Print => write!(f, "print"),
            Sexp::If => write!(f, "if"),
            Sexp::Eq => write!(f, "eq"),
            Sexp::Quote => write!(f, "quote"),
            Sexp::Cons => write!(f, "cons"),
            Sexp::Car => write!(f, "car"),
            Sexp::Cdr => write!(f, "cdr"),
            Sexp::Lambda | Sexp::CapturedLambda(_) => write!(f, "λ"),
            Sexp::Macro => write!(f, "macro"),
            Sexp::RustFn(_) => write!(f, "rustfn"),
            Sexp::Eval => write!(f, "eval"),
            Sexp::Define => write!(f, "define"),
            Sexp::Require => write!(f, "require"),
            Sexp::Provide => write!(f, "provide"),
            Sexp::Nil => write!(f, "()"),
            Sexp::I32(n) => write!(f, "{}", n),
            Sexp::Char(c) => write!(f, "'{}'", c),
            Sexp::SString(s) => write!(f, "\"{}\"", s),
            Sexp::Bool(b) => write!(f, "{}", b),
            Sexp::Identifier(ident) => write!(f, "{}", ident),
            Sexp::Form(cons) => {
                write!(f, "(")?;
                write!(f, "{}", cons)?;
                write!(f, ")")
            }
        }
    }
}

impl From<i32> for Sexp {
    fn from(value: i32) -> Self {
        Self::I32(value)
    }
}

impl From<String> for Sexp {
    fn from(value: String) -> Self {
        Self::SString(value)
    }
}

impl From<bool> for Sexp {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<char> for Sexp {
    fn from(value: char) -> Self {
        Self::Char(value)
    }
}

impl Display for Cons {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.car.as_ref(), self.cdr.as_ref()) {
            (Sexp::Form(_), Sexp::Form(tail_cons)) => {
                write!(f, "{} {}", self.car.clone(), tail_cons)
            }
            (car, Sexp::Form(tail_cons)) => {
                write!(f, "{} {}", car, tail_cons)
            }
            (Sexp::Form(_), Sexp::Nil) => {
                write!(f, "{}", self.car.clone())
            }
            (Sexp::Form(_), cdr) => {
                write!(f, "{} . {}", self.car.clone(), cdr)
            }
            (car, Sexp::Nil) => {
                write!(f, "{}", car)
            }
            (car, cdr) => {
                write!(f, "{} . {}", car, cdr)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::parse::parse_sexp;
    use crate::sexp::{Cons, Sexp};

    #[test]
    fn nil_is_nil() {
        assert!(Sexp::Nil.is_nil())
    }

    #[test]
    fn nil_form_is_nil() {
        assert!(Sexp::Form(Cons {
            car: Sexp::nil(),
            cdr: Sexp::nil()
        })
        .is_nil())
    }

    #[test]
    fn check_quoted() {
        let expr = parse_sexp("(1 2 3)").unwrap().1;
        assert!(!expr.is_quoted());
        let expr = parse_sexp("'(1 2 3)").unwrap().1;
        assert!(expr.is_quoted());
    }

    #[test]
    fn get_quoted() {
        let expr = parse_sexp("'(1 2 3)").unwrap().1;
        assert!(expr.is_quoted());
        let expected = parse_sexp("(1 2 3)").unwrap().1;
        assert_eq!(expr.get_quoted(), Some(expected));
    }

    #[test]
    fn get_quoted_atom() {
        let expr = parse_sexp("'a").unwrap().1;
        assert!(expr.is_quoted());
        let expected = parse_sexp("a").unwrap().1;
        assert_eq!(expr.get_quoted(), Some(expected));
    }
}
