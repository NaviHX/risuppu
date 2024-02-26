use crate::pre_function;

mod and;
mod or;
mod not;

crate::std_library!(
    bool,
    (and::and, "__builtin_and", pre_function),
    (or::or, "__builtin_or", pre_function),
    (not::not, "__builtin_not", pre_function)
);
