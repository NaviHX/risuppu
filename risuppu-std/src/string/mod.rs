use crate::pre_function;

mod concat;

super::std_library!(
    string,
    (concat::concat, "__builtin_concat", pre_function)
);
