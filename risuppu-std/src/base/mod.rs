use and_then::eval_cond;
use crate::pre_function;

super::std_library!(
    base,
    (seq, "seq", pre_function),
    (r#do, "do"),
    (and_then, "and-then", eval_cond),
    (r#let, "let"),
    (cond, "cond")
);
