pub mod frame;
pub mod env;
use gc::Gc;
use frame::Frame;
pub use env::Env;

use crate::sexp::{Cons, Ptr, Sexp};

mod module;
use self::module::{process_require, process_provide};

pub fn evaluate(mut sexp: Ptr<Sexp>, env: &mut Env) -> Ptr<Sexp> {
    #[cfg(debug_assertions)]
    println!("Eval: {}", sexp);
    #[cfg(debug_assertions)]
    let orig_sexp = sexp.clone();

    env.push_frame();
    let cur_top = env.top_frame();

    let evaluated = loop {
        // The `break`ed val is the return val,

        // If you want to inspect the sexp when debugging,
        // uncomment the following line.
        #[cfg(debug_assertions)]
        let s = sexp.to_string();

        #[cfg(debug_assertions)]
        println!("=> {}", s);

        sexp = match sexp.as_ref() {
            Sexp::Form(list) => {
                let car = list.car.clone();
                let cdr = list.cdr.clone();
                match car.as_ref() {
                    Sexp::Read => process_read(cdr, env),
                    Sexp::Print => process_print(cdr, env),
                    Sexp::If => process_if(cdr, env),
                    Sexp::Eq => process_eq(cdr, env),
                    Sexp::Quote => break cdr.car(),
                    Sexp::Cons => break process_cons(cdr, env),
                    Sexp::Car => break process_car(cdr, env),
                    Sexp::Cdr => break process_cdr(cdr, env),
                    Sexp::Lambda => {
                        // Push a new frame to avoid modifing the captured environment.
                        let current_frame_ptr = env.top_frame().expect("No stack frame!");
                        let current_frame_ptr = Frame::push(Some(current_frame_ptr));
                        let new_lambda = Sexp::lambda_capture(current_frame_ptr);
                        let new_expr = Sexp::cons(new_lambda, cdr);
                        break new_expr;
                    }
                    Sexp::Macro => break sexp.clone(),
                    Sexp::CapturedLambda(_) => break sexp.clone(),
                    Sexp::Eval => evaluate(cdr, env),
                    Sexp::Define => process_define(cdr, env),
                    Sexp::Require => process_require(cdr, env),
                    Sexp::Provide => process_provide(cdr, env),

                    // Do nothing if the first sexp is nil.
                    Sexp::Nil => break car.clone(),

                    // Replace the identity with its defination,
                    // and then evaluate the whole expression again.
                    Sexp::Identifier(ident) => match env.get(ident.as_str()) {
                        Some(new_car) => Ptr::new(Sexp::Form(Cons::new(new_car, cdr))),
                        None => panic!("Cannot find ident: {ident}"),
                    },

                    // Evaluate the CAR and Replace it with the result.
                    // Then evaluate the whole expression again.
                    Sexp::Form(list) => {
                        match list.car.as_ref() {
                            Sexp::Lambda => {
                                let args = eval_args(cdr, env);
                                apply_list_to(args, car, env)
                            }
                            Sexp::CapturedLambda(captured_frame) => {
                                let args = eval_args(cdr, env);
                                env.set_frame_ptr(Some(captured_frame.clone()));
                                apply_list_to(args, car, env)
                            }
                            Sexp::Macro => apply_list_to(cdr, car, env),
                            _ => {
                                let new_car = env.evaluate(car.clone());
                                Sexp::cons(new_car, cdr)
                            }
                        }
                    }

                    // Apply the CDR to the Rust function.
                    Sexp::RustFn(f) => {
                        f.call(cdr, env)
                    }

                    exp => {
                        println!("Error: {exp:?} is not appliable!");
                        break sexp.clone();
                    }
                }
            }

            Sexp::Identifier(ident) => match env.get(ident.as_str()) {
                Some(sexp) => break sexp,
                None => panic!("Cannot find ident: {ident}"),
            },

            _ => break sexp.clone(),
        }
    };

    // Restore the stack.
    env.set_frame_ptr(cur_top);
    env.pop_frame();

    #[cfg(debug_assertions)]
    println!("Evaluated: {} => {}", orig_sexp, evaluated);
    evaluated
}

fn process_car(body: Gc<Sexp>, env: &mut Env) -> Gc<Sexp> {
    let arg = body.car();
    let arg = env.evaluate(arg);
    arg.car()
}

fn process_cdr(body: Gc<Sexp>, env: &mut Env) -> Gc<Sexp> {
    let arg = body.car();
    let arg = env.evaluate(arg);
    arg.cdr()
}

pub fn process_if(body: Ptr<Sexp>, env: &mut Env) -> Ptr<Sexp> {
    let condition = body.car();
    let if_branch = body.cdr().car();
    let else_branch = body.cdr().cdr().car();

    if let Sexp::Bool(true) = evaluate(condition, env).as_ref() {
        if_branch
    } else {
        else_branch
    }
}

pub fn process_eq(body: Ptr<Sexp>, env: &mut Env) -> Ptr<Sexp> {
    let pre = evaluate(body.car(), env);
    let mut remaining = body.cdr();

    loop {
        let car = remaining.car();
        let car = evaluate(car, env);
        if !car.is_nil() {
            if car != pre {
                break Ptr::new(Sexp::Bool(false));
            }

            remaining = remaining.cdr();
        } else {
            break Ptr::new(Sexp::Bool(true));
        }
    }
}

pub fn process_cons(body: Ptr<Sexp>, env: &mut Env) -> Ptr<Sexp> {
    let first = evaluate(body.car(), env);
    let second = evaluate(body.cdr().car(), env);

    Ptr::new(Sexp::Form(Cons {
        car: first,
        cdr: second,
    }))
}

pub fn process_define(body: Ptr<Sexp>, env: &mut Env) -> Ptr<Sexp> {
    let identity = body.car();
    let defination = env.evaluate(body.cdr().car());

    if let Sexp::Identifier(ident) = identity.as_ref() {
        env.set_global(ident, defination)
    }

    Ptr::new(Sexp::Nil)
}

pub fn process_read(body: Ptr<Sexp>, _env: &mut Env) -> Ptr<Sexp> {
    fn trim_newline(s: &mut String) {
        if s.ends_with('\n') {
            s.pop();
            if s.ends_with('\r') {
                s.pop();
            }
        }
    }

    use std::io::stdin;
    let f = stdin();
    let mut buf = String::new();
    f.read_line(&mut buf).unwrap();
    trim_newline(&mut buf);
    let unescaped = unescaper::unescape(&buf).unwrap_or("ERROR when unescaping".to_string());

    let arg = Ptr::new(Sexp::SString(unescaped));
    let func = body.car();
    Sexp::from_vec(vec![func, arg])
}

pub fn process_print(body: Ptr<Sexp>, env: &mut Env) -> Ptr<Sexp> {
    let content = evaluate(body.car(), env);
    let func = body.cdr().car();
    if let Sexp::SString(content) = content.as_ref() {
        print!("{}", content);
    } else {
        print!("{}", content);
    }
    Sexp::from_vec(vec![func])
}

pub fn eval_args(args: Ptr<Sexp>, env: &mut Env) -> Ptr<Sexp> {
    let evaluated_args: Vec<_> = Sexp::iter(args).map(|a| env.evaluate(a)).collect();
    Sexp::from_vec(evaluated_args)
}

pub fn apply_list_to(mut args: Ptr<Sexp>, expr: Ptr<Sexp>, env: &mut Env) -> Ptr<Sexp> {
    let first_token = expr.car();
    let mut params = expr.cdr().car();
    let body = expr.cdr().cdr().car();

    if !first_token.is_lambda() && !first_token.is_macro() {
        return expr;
    }

    while !args.is_nil() {
        let (first_param, remaining_params) = (params.car(), params.cdr());
        let (arg, remaining_args) = (args.car(), args.cdr());
        if let Sexp::Identifier(ident) = first_param.as_ref() {
            match first_token.as_ref() {
                Sexp::Macro | Sexp::Lambda | Sexp::CapturedLambda(_) => env.set(ident, arg),
                _ => panic!("Cannot apply to a non-lambda expr!"),
            }
        }

        params = remaining_params;
        args = remaining_args;
    }

    if params.is_nil() {
        body
    } else {
        Sexp::from_vec(vec![first_token, params, body])
    }
}

#[cfg(test)]
mod test {
    use crate::sexp::{Sexp, parse::parse_sexp};

    use super::Env;

    #[test]
    fn hello_world() {
        let expr = Sexp::cons(
            Sexp::wrap(Sexp::Print),
            Sexp::cons(
                Sexp::wrap(Sexp::SString("Hello World".to_string())),
                Sexp::nil(),
            ),
        );
        let mut env = Env::new();
        env.evaluate(expr);
    }

    #[test]
    fn eval_if() {
        let mut env = Env::new();

        let sexp = Sexp::from_vec(vec![
            Sexp::r#if(),
            Sexp::bool(true),
            Sexp::int(1),
            Sexp::int(2),
        ]);
        let res = env.evaluate(sexp);
        assert_eq!(res, Sexp::int(1));

        let sexp = Sexp::from_vec(vec![
            Sexp::r#if(),
            Sexp::bool(false),
            Sexp::int(1),
            Sexp::int(2),
        ]);
        let res = env.evaluate(sexp);
        assert_eq!(res, Sexp::int(2));
    }

    #[test]
    fn eval_define() {
        let mut env = Env::new();
        let definition = Sexp::int(1);
        let identifier = Sexp::identifier("ident");
        let sexp = Sexp::from_vec(vec![Sexp::define(), identifier.clone(), definition.clone()]);

        env.evaluate(sexp);
        assert!(env.get("ident").is_some());
        assert_eq!(env.get("ident").unwrap(), definition);
    }

    #[test]
    fn eval_lambda() {
        let mut env = Env::new();
        let lambda = Sexp::from_vec(vec![
            Sexp::lambda(),
            Sexp::from_vec(vec![Sexp::identifier("a")]),
            Sexp::identifier("a"),
        ]);
        let expr = Sexp::from_vec(vec![lambda, Sexp::int(1)]);

        let res = env.evaluate(expr);
        assert_eq!(res, Sexp::int(1));
    }

    #[test]
    fn print_and_then_test() {
        let mut env = Env::new();
        let and_then_expr = Sexp::from_vec(vec![Sexp::lambda(), Sexp::nil(), Sexp::int(1)]);
        let print_expr = Sexp::from_vec(vec![Sexp::print(), Sexp::string("Print"), and_then_expr]);

        let res = env.evaluate(print_expr);
        assert_eq!(res, Sexp::int(1));
    }

    #[test]
    fn eval_none_param_lambda() {
        let mut env = Env::new();
        let sexp = Sexp::from_vec(vec![Sexp::from_vec(vec![
            Sexp::lambda(),
            Sexp::nil(),
            Sexp::int(1),
        ])]);
        let res = env.evaluate(sexp);
        assert_eq!(res, Sexp::int(1));
    }

    #[test]
    fn eval_nested_lambda() {
        let mut env = Env::new();
        let inner_lambda = Sexp::from_vec(vec![Sexp::lambda(), Sexp::nil(), Sexp::identifier("a")]);
        let outer_lambda = Sexp::from_vec(vec![
            Sexp::lambda(),
            Sexp::from_vec(vec![Sexp::identifier("a")]),
            inner_lambda,
        ]);
        let expr = Sexp::from_vec(vec![Sexp::from_vec(vec![outer_lambda, Sexp::int(1)])]);
        let res = env.evaluate(expr);
        assert_eq!(res, Sexp::int(1));
    }

    #[test]
    fn eval_macro() {
        let mut env = Env::new();
        let lambda = Sexp::from_vec(vec![
            Sexp::r#macro(),
            Sexp::from_vec(vec![Sexp::identifier("a")]),
            Sexp::identifier("a"),
        ]);
        let expr = Sexp::from_vec(vec![lambda, Sexp::int(1)]);

        let res = env.evaluate(expr);
        assert_eq!(res, Sexp::int(1));
    }

    #[test]
    fn test_macro_quoted_arg() {
        let mut env = Env::new();
        env.set_global(Sexp::identifier("a"), Sexp::int(1));
        assert_eq!(env.get("a").unwrap(), Sexp::int(1));

        let arg = Sexp::from_vec([Sexp::define(), Sexp::identifier("a"), Sexp::int(2)]);
        // ((macro (b)) (define a 2))
        let macro_expr = Sexp::from_vec([Sexp::from_vec([Sexp::r#macro(), Sexp::from_vec([Sexp::identifier("b")])]), arg.clone()]);
        // ((lambda (b)) (define a 2))
        let lambda_expr = Sexp::from_vec([Sexp::from_vec([Sexp::lambda(), Sexp::from_vec([Sexp::identifier("b")])]), arg.clone()]);

        env.evaluate(macro_expr);
        assert_eq!(env.get("a").unwrap(), Sexp::int(1));
        env.evaluate(lambda_expr);
        assert_eq!(env.get("a").unwrap(), Sexp::int(2));
    }

    #[test]
    fn eval_rust_fn() {
        let mut env = Env::new();
        let mut v = 0;
        let f = move |_, _: &mut Env| {
            v += 1;
            Sexp::int(v)
        };
        let rustfn_expr = unsafe { Sexp::rust_fn(f) };
        let expr = Sexp::from_vec([rustfn_expr]);

        assert_eq!(env.evaluate(expr.clone()), Sexp::int(1));
        assert_eq!(env.evaluate(expr.clone()), Sexp::int(2));
        assert_eq!(env.evaluate(expr.clone()), Sexp::int(3));
    }

    #[test]
    fn eval_car_and_cdr() {
        let mut env = Env::new();
        let list = Sexp::from_vec([Sexp::int(1), Sexp::int(2)]);
        let car_expr = Sexp::from_vec([Sexp::car_token(), list.clone()]);
        let cdr_expr = Sexp::from_vec([Sexp::cdr_token(), list.clone()]);

        assert_eq!(env.evaluate(car_expr), Sexp::int(1));
        assert_eq!(env.evaluate(cdr_expr), Sexp::from_vec([Sexp::int(2)]));
    }

}
