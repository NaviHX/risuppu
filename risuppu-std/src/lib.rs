use risuppu::{sexp::{Sexp, Ptr}, semantic::Env};

pub mod base;
#[cfg(feature = "arithmetic")]
pub mod arithmetic;
#[cfg(feature = "string")]
pub mod string;
#[cfg(feature = "bool")]
pub mod bool;

pub use paste::paste;

#[allow(unused_macros)]
#[macro_export]
macro_rules! std_library {
    ($mod_name:ident, $(($function_name:ident, $($r:tt)*)),*) => {
        $(mod $function_name;)*

        $crate::paste! {
            pub fn [<load_ $mod_name>](env: &mut risuppu::semantic::Env) {
                $($crate::load_fn!(env, $function_name, $($r)*));*
            }
        }
    }
}

#[macro_export]
macro_rules! load_fn {
    ($env:ident, $function_name:ident, $rt_name:literal, $pre_function:ident) => {
        $env.set_global($rt_name, unsafe {
            risuppu::sexp::Sexp::rust_fn_with_preprocess(
                $function_name::$function_name,
                $pre_function,
            )
        })
    };
    ($env:ident, $function_name:ident, $rt_name:literal) => {
        $env.set_global($rt_name, unsafe {
            risuppu::sexp::Sexp::rust_fn($function_name::$function_name)
        })
    };
}

pub fn pre_function(args: Ptr<Sexp>, env: &mut Env) -> Ptr<Sexp> {
    let args: Vec<_> = Sexp::iter(args).map(|s| env.evaluate(s)).collect();
    Sexp::from_vec(args)
}

#[cfg(test)]
mod test {
    use risuppu::{sexp::{parse::parse_sexp, Sexp}, semantic::Env};

    use crate::arithmetic::load_arithmetic;

    #[test]
    fn omega() {
        let sum = parse_sexp("(lambda (s p n) (if (eq n 0) p (s s (+ p n) (- n 1))))").unwrap().1;
        let omega = parse_sexp("(lambda (f) (f f))").unwrap().1;
        let mut env = Env::new();
        load_arithmetic(&mut env);
        env.evaluate(Sexp::from_vec([Sexp::define(), Sexp::identifier("sum"), sum]));
        env.evaluate(Sexp::from_vec([Sexp::define(), Sexp::identifier("omega"), omega]));
        let sum5 = parse_sexp("((omega sum) 0 5)").unwrap().1;
        let res = env.evaluate(sum5);
        assert_eq!(res, Sexp::int(15));
    }

    #[test]
    fn y() {
        let sum = parse_sexp("(lambda (s p n) (if (eq n 0) p (s (+ p n) (- n 1))))").unwrap().1;
        let omega = parse_sexp("(lambda (f) (f f))").unwrap().1;
        let y = parse_sexp("(lambda (f) (omega (lambda (g) (f (g g)))))").unwrap().1;
        let mut env = Env::new();
        load_arithmetic(&mut env);
        env.evaluate(Sexp::from_vec([Sexp::define(), Sexp::identifier("sum"), sum]));
        env.evaluate(Sexp::from_vec([Sexp::define(), Sexp::identifier("omega"), omega]));
        env.evaluate(Sexp::from_vec([Sexp::define(), Sexp::identifier("y"), y]));
        let sum5 = parse_sexp("((y sum) 0 5)").unwrap().1;
        let res = env.evaluate(sum5);
        assert_eq!(res, Sexp::int(15));
    }
}
