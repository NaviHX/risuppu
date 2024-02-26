use and_then::eval_cond;
use crate::pre_function;

mod seq;
mod r#do;
mod and_then;
mod r#let;
mod cond;

super::std_library!(
    base,
    (seq::seq, "seq", pre_function),
    (r#do::r#do, "do"),
    (and_then::and_then, "and-then", eval_cond),
    (r#let::r#let, "let"),
    (cond::cond, "cond")
);
