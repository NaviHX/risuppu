use and_then::eval_cond;

super::std_library!(
    base,
    (seq, "seq"),
    (r#do, "do"),
    (and_then, "and-then", eval_cond),
    (r#let, "let")
);
