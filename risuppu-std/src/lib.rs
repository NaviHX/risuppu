pub mod string;
pub mod arithmetic;
pub mod seq;

#[allow(unused_macros)]
#[macro_export]
macro_rules! std_library {
    ($mod_name:ident, $(($function_name:ident, $rt_name:ident)),*) => {
        $(mod $function_name;)*
        $(use $function_name::$function_name;)*

        pub fn $mod_name(env: &mut risuppu::semantic::Env) {
            $(env.set_global(stringify!(rt_name), unsafe { risuppu::sexp::Sexp::rust_fn($function_name) });)*
        }
    }
}
