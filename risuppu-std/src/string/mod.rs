use crate::pre_function;

mod concat;

super::std_library!(
    string,
    (concat::concat, "concat", pre_function)
);
