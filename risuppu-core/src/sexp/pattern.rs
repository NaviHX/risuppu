use super::{Ptr, Sexp};

/// # Pattern
/// TODO
#[derive(Clone, Debug)]
pub enum Pattern {
    Literal(Ptr<Sexp>),
    Binding(String),
    List(Ptr<Sexp>),
    Nil,
}

impl Pattern {
    pub fn new(expr: Ptr<Sexp>) -> Self {
        match expr.as_ref() {
            Sexp::Identifier(ident) => Pattern::Binding(ident.clone()),
            Sexp::Nil => Pattern::Nil,
            Sexp::Form(_)
                if !(expr.is_quoted()
                    && expr
                        .get_quoted()
                        .map(|e| e.is_identifier())
                        .unwrap_or(false)) =>
            {
                Pattern::List(expr)
            }
            _ => {
                if expr.is_quoted()
                    && expr
                        .get_quoted()
                        .map(|e| e.is_identifier())
                        .unwrap_or(false)
                {
                    Pattern::Literal(expr.get_quoted().unwrap())
                } else {
                    Pattern::Literal(expr)
                }
            }
        }
    }
}

impl From<Ptr<Sexp>> for Pattern {
    fn from(value: Ptr<Sexp>) -> Self {
        Self::new(value)
    }
}

#[derive(Clone, Debug)]
pub struct MatchError {
    #[allow(dead_code)]
    expected: Pattern,
    #[allow(dead_code)]
    existed: Ptr<Sexp>,
}

impl MatchError {
    pub fn new(expected: Pattern, existed: Ptr<Sexp>) -> Self {
        Self { expected, existed }
    }
}

type MatchResult = Result<Binding, MatchError>;

pub struct Binding {
    inner: Vec<(String, Ptr<Sexp>)>,
}

impl Binding {
    pub fn new<T>(bindings: T) -> Self
    where
        T: IntoIterator,
        <T as std::iter::IntoIterator>::IntoIter: Iterator,
        <T as std::iter::IntoIterator>::Item: Into<(String, Ptr<Sexp>)>,
    {
        Self {
            inner: bindings.into_iter().map(Into::into).collect(),
        }
    }

    pub fn empty() -> Self {
        Self { inner: vec![] }
    }

    pub fn add_binding(&mut self, identifier: impl ToString, value: Ptr<Sexp>) {
        self.inner.push((identifier.to_string(), value))
    }

    pub fn extend_binding(&mut self, bindings: Self) {
        self.inner.extend(bindings.inner)
    }

    pub fn get_binding(self) -> Vec<(String, Ptr<Sexp>)> {
        self.inner
    }
}

impl Pattern {
    /// Check if the expression matches the pattern.
    pub fn matches(&self, expr: Ptr<Sexp>) -> bool {
        self.bind(expr).is_ok()
    }

    /// Bind the values in the expression with the identifiers in the patter.
    pub fn bind(&self, expr: Ptr<Sexp>) -> MatchResult {
        match (self, expr.as_ref()) {
            (Pattern::Literal(_), Sexp::Nil | Sexp::Form(_)) => {
                Err(MatchError::new(self.clone(), expr))
            }
            (Pattern::Literal(pattern_literal), _) => {
                if *pattern_literal == expr {
                    Ok(Binding::empty())
                } else {
                    Err(MatchError::new(self.clone(), expr))
                }
            }
            (Pattern::Binding(_), Sexp::Nil) => Err(MatchError::new(self.clone(), expr)),
            (Pattern::Binding(ident), _) => {
                if !expr.is_nil() {
                    Ok(Binding::new([(ident.clone(), expr)]))
                } else {
                    Err(MatchError::new(self.clone(), expr))
                }
            }
            (Pattern::List(inner_list), Sexp::Form(_)) if !expr.is_nil() => {
                let pattern_list = inner_list.clone();
                let mut expr_iter = Sexp::iter(expr);
                let mut bindings = Binding::empty();
                for pattern in Sexp::iter(pattern_list).map(Pattern::from) {
                    let expr = expr_iter
                        .next()
                        .ok_or_else(|| MatchError::new(pattern.clone(), Sexp::nil()))?;
                    let b = pattern.bind(expr)?;
                    bindings.extend_binding(b);
                }

                let remaining = expr_iter.next();
                if let Some(remaining) = remaining {
                    Err(MatchError { expected: Pattern::Nil, existed: remaining })
                } else {
                    Ok(bindings)
                }
            }
            (Pattern::List(_), _) => Err(MatchError::new(self.clone(), expr)),
            (Pattern::Nil, Sexp::Nil | Sexp::Form(_)) => {
                if expr.is_nil() {
                    Ok(Binding::empty())
                } else {
                    Err(MatchError::new(self.clone(), expr))
                }
            }
            (Pattern::Nil, _) => Err(MatchError::new(self.clone(), expr)),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::sexp::{parse::parse_sexp, Sexp};

    use super::{MatchError, Pattern};

    #[test]
    fn match_nil_pattern() -> Result<(), MatchError> {
        let pattern: Pattern = parse_sexp("()").unwrap().1.into();
        let expr = parse_sexp("()").unwrap().1;
        pattern.bind(expr).map(|_| ())
    }

    #[test]
    fn match_int() -> Result<(), MatchError> {
        let pattern: Pattern = parse_sexp("1").unwrap().1.into();
        let expr = parse_sexp("1").unwrap().1;
        pattern.bind(expr).map(|_| ())
    }

    #[test]
    fn match_atom() -> Result<(), MatchError> {
        let pattern: Pattern = parse_sexp("define").unwrap().1.into();
        let expr = parse_sexp("define").unwrap().1;
        pattern.bind(expr).map(|_| ())
    }

    #[test]
    fn match_identifier() -> Result<(), MatchError> {
        let pattern: Pattern = parse_sexp("'ident").unwrap().1.into();
        let expr = parse_sexp("ident").unwrap().1;
        pattern.bind(expr).map(|_| ())
    }

    #[test]
    fn match_list() -> Result<(), MatchError> {
        let pattern: Pattern = parse_sexp("'(1 2 3)").unwrap().1.into();
        let expr = parse_sexp("'(1 2 3)").unwrap().1;
        pattern.bind(expr).map(|_| ())
    }

    #[test]
    fn match_embedded_list() -> Result<(), MatchError> {
        let pattern: Pattern = parse_sexp("'(1 2 3 (1 2 3))").unwrap().1.into();
        let expr = parse_sexp("'(1 2 3 (1 2 3))").unwrap().1;
        pattern.bind(expr).map(|_| ())
    }

    #[test]
    fn bind() {
        let pattern: Pattern = parse_sexp("n").unwrap().1.into();
        let expr = parse_sexp("1").unwrap().1;
        let binding = pattern.bind(expr).unwrap();
        let binding = binding.get_binding();
        assert_eq!(binding[0], ("n".to_string(), Sexp::int(1)))
    }

    #[test]
    fn bind_many() {
        let pattern: Pattern = parse_sexp("(a b 1 2 c)").unwrap().1.into();
        let expr = parse_sexp("(1 2 1 2 3)").unwrap().1;
        let binding = pattern.bind(expr).unwrap();
        let binding = binding.get_binding();
        assert_eq!(binding[0], ("a".to_string(), Sexp::int(1)));
        assert_eq!(binding[1], ("b".to_string(), Sexp::int(2)));
        assert_eq!(binding[2], ("c".to_string(), Sexp::int(3)));
    }

    #[test]
    fn bind_embedded() {
        let pattern: Pattern = parse_sexp("((a b) 1 2 c)").unwrap().1.into();
        let expr = parse_sexp("((1 2) 1 2 3)").unwrap().1;
        let binding = pattern.bind(expr).unwrap();
        let binding = binding.get_binding();
        assert_eq!(binding[0], ("a".to_string(), Sexp::int(1)));
        assert_eq!(binding[1], ("b".to_string(), Sexp::int(2)));
        assert_eq!(binding[2], ("c".to_string(), Sexp::int(3)));
    }

    #[test]
    fn bind_do() {
        let pattern: Pattern = parse_sexp("('-> m cont)").unwrap().1.into();
        let expr = parse_sexp("(-> read print)").unwrap().1;
        let binding = pattern.bind(expr).unwrap();
        let binding = binding.get_binding();
        assert_eq!(binding[0], ("m".to_string(), Sexp::read()));
        assert_eq!(binding[1], ("cont".to_string(), Sexp::print()));
    }

    #[test]
    fn match_identifier_failure() {
        let pattern: Pattern = parse_sexp("'->").unwrap().1.into();
        let expr = parse_sexp("'->").unwrap().1;
        assert!(!pattern.matches(expr))
    }

    #[test]
    fn exhausted_match() {
        let pattern: Pattern = parse_sexp("(a)").unwrap().1.into();
        let expr = parse_sexp("(1 2)").unwrap().1;
        assert!(!pattern.matches(expr))
    }
}
