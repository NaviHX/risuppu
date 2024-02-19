use crate::pre_function;

crate::std_library!(
    bool,
    (and, "and", pre_function),
    (or, "or", pre_function),
    (not, "not", pre_function)
);
