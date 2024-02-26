use risuppu::{
    semantic::Env,
    sexp::{Ptr, Sexp},
};

use crate::id;

mod flat_map;
mod fold;
mod map;

crate::std_library!(
    list,
    (create_list, "__builtin_list", crate::pre_function),
    (id, "__builtin_fold", fold::fold),
    (id, "__builtin_map", map::map),
    (id, "__builtin_flat-map", flat_map::flat_map)
);

pub fn quote(args: Ptr<Sexp>) -> Ptr<Sexp> {
    Sexp::from_vec([Sexp::quote(), args])
}

pub fn create_list(args: Ptr<Sexp>, env: &mut Env) -> Ptr<Sexp> {
    quote(Sexp::from_vec(
        Sexp::iter(args)
            .map(|arg| env.evaluate(arg))
            .collect::<Vec<_>>(),
    ))
}
