pub mod parse;
use gc::{Finalize, Gc, Trace};
use std::fmt::Display;

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

    // Lambda
    Lambda,

    // Evaluate
    Eval,

    // Define
    Define,

    // Data
    I32(i32),
    Char(char),
    SString(String),
    Bool(bool),

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
    keyword_wrapper!(lambda, Sexp::Lambda);
    keyword_wrapper!(eval, Sexp::Eval);
    keyword_wrapper!(define, Sexp::Define);

    literal_wrapper!(int, i32, Sexp::I32);
    literal_wrapper!(r#char, char, Sexp::Char);
    literal_wrapper!(r#bool, bool, Sexp::Bool);

    pub fn string(s: impl ToString) -> Ptr<Self> {
        Sexp::wrap(Sexp::SString(s.to_string()))
    }

    pub fn identifier(s: impl ToString) -> Ptr<Self> {
        Sexp::wrap(Sexp::Identifier(s.to_string()))
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
            Sexp::Lambda => write!(f, "λ"),
            Sexp::Eval => write!(f, "eval"),
            Sexp::Define => write!(f, "define"),
            Sexp::Nil => write!(f, "()"),
            Sexp::I32(n) => write!(f, "{}", n),
            Sexp::Char(c) => write!(f, "'{}'", c),
            Sexp::SString(s) => write!(f, "\"{}\"", s),
            Sexp::Bool(b) => write!(f, "{}", b),
            Sexp::Identifier(ident) => write!(f, "#{}", ident),
            Sexp::Form(cons) => {
                write!(f, "(")?;
                write!(f, "{}", cons)?;
                write!(f, ")")
            }
        }
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
