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
    (create_list, "list", crate::pre_function),
    (id, "fold", fold::fold),
    (id, "map", map::map),
    (id, "flat-map", flat_map::flat_map)
);

pub fn create_list(args: Ptr<Sexp>, env: &mut Env) -> Ptr<Sexp> {
    Sexp::from_vec(
        Sexp::iter(args)
            .map(|arg| env.evaluate(arg))
            .collect::<Vec<_>>(),
    )
}
