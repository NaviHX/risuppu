use crate::pre_function;

mod and;
mod or;
mod not;

crate::std_library!(
    bool,
    (and::and, "and", pre_function),
    (or::or, "or", pre_function),
    (not::not, "not", pre_function)
);
