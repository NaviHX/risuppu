use std::collections::HashMap;

use crate::sexp::{Cons, Ptr, Sexp};

pub struct Env {
    symbol_table: HashMap<String, Ptr<Sexp>>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            symbol_table: HashMap::new(),
        }
    }

    pub fn get(&self, identity: impl AsRef<str>) -> Option<Ptr<Sexp>> {
        self.symbol_table.get(identity.as_ref()).cloned()
    }

    pub fn set(&mut self, identity: impl ToString, expr: Ptr<Sexp>) {
        self.symbol_table.insert(identity.to_string(), expr.clone());
    }

    pub fn evaluate(&mut self, expr: Ptr<Sexp>) -> Ptr<Sexp> {
        evaluate(expr, self)
    }
}

impl Default for Env {
    fn default() -> Self {
        Self::new()
    }
}

pub fn evaluate(mut sexp: Ptr<Sexp>, env: &mut Env) -> Ptr<Sexp> {
    loop {
        // The `break`ed val is the return val,
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
                    Sexp::Lambda => break sexp,
                    Sexp::Eval => evaluate(cdr, env),
                    Sexp::Define => process_define(cdr, env),

                    // Do nothing if the first sexp is nil.
                    Sexp::Nil => break car.clone(),

                    // Replace the identity with its defination,
                    // and then evaluate the whole expression again.
                    Sexp::Identifier(ident) => match env.get(ident.as_str()) {
                        Some(new_car) => Ptr::new(Sexp::Form(Cons::new(new_car, cdr))),
                        None => Ptr::new(Sexp::Nil),
                    },

                    // Evaluate the CAR and Replace it with the result.
                    // Then evaluate the whole expression again.
                    Sexp::Form(list) => {
                        if list.car == Sexp::lambda() {
                            apply_list_to_lambda(cdr, car, env)
                        } else {
                            let new_car = evaluate(car.clone(), env);
                            Ptr::new(Sexp::Form(Cons::new(new_car, cdr)))
                        }
                    }

                    exp => {
                        println!("Error: {exp:?} is not appliable!");
                        break sexp.clone();
                    }
                }
            }

            Sexp::Identifier(ident) => match env.get(ident.as_str()) {
                Some(sexp) => sexp,
                None => break Ptr::new(Sexp::Nil),
            },

            _ => break sexp.clone(),
        }
    }
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
    let pre = body.car();
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
        env.set(ident, defination)
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

    let arg = Ptr::new(Sexp::SString(buf));
    let func = body.car();
    Sexp::from_vec(vec![func, arg])
}

pub fn process_print(body: Ptr<Sexp>, _env: &mut Env) -> Ptr<Sexp> {
    let content = body.car();
    let func = body.cdr().car();
    if let Sexp::SString(content) = content.as_ref() {
        println!("{}", content);
    } else {
        println!("{}", content);
    }
    Sexp::from_vec(vec![func])
}

pub fn apply_to_lambda(arg: Ptr<Sexp>, lambda: Ptr<Sexp>, env: &mut Env) -> Ptr<Sexp> {
    let lambda_token = lambda.car();
    if *lambda_token != Sexp::Lambda {
        return lambda;
    }

    let lambda_params = lambda.cdr().car();
    let lambda_body = lambda.cdr().cdr().car();
    let lambda_arg = evaluate(arg, env);
    let (first_param, remaining_params) = (lambda_params.car(), lambda_params.cdr());

    let applied_body = if let Sexp::Identifier(ident) = first_param.as_ref() {
        apply_body(lambda_arg, ident.as_ref(), lambda_body)
    } else {
        lambda_body
    };

    if remaining_params.is_nil() {
        applied_body
    } else {
        Sexp::cons(
            lambda_token,
            Sexp::cons(remaining_params, Sexp::cons(applied_body, Sexp::nil())),
        )
    }
}

pub fn apply_list_to_lambda(
    mut args: Ptr<Sexp>,
    mut lambda: Ptr<Sexp>,
    env: &mut Env,
) -> Ptr<Sexp> {
    while !args.is_nil() {
        let arg = args.car();
        lambda = apply_to_lambda(arg, lambda, env);
        args = args.cdr();
    }

    // Guard if no params in lambda
    if lambda.car() == Sexp::lambda() && lambda.cdr().car().is_nil() {
        lambda.cdr().cdr().car()
    } else {
        lambda
    }
}

fn apply_body(arg: Ptr<Sexp>, param: &str, body: Ptr<Sexp>) -> Ptr<Sexp> {
    // TODO: `apply_body` needs rewrite.
    // Defer the substitution to eval-time.
    // Maybe I should create a new type of Sexp called "Sub"
    // which will substitute the param with the arg when evaluate itself.
    if body.is_nil() {
        return body;
    }

    match body.as_ref() {
        Sexp::Identifier(ident) => {
            if ident.as_str() == param {
                arg
            } else {
                body
            }
        }
        Sexp::Form(Cons { car, cdr }) => Sexp::cons(
            apply_body(arg.clone(), param, car.clone()),
            apply_body(arg, param, cdr.clone()),
        ),
        _ => body,
    }
}

#[cfg(test)]
mod test {
    use crate::sexp::Sexp;

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
        let sexp = Sexp::from_vec(vec![Sexp::from_vec(vec![Sexp::lambda(), Sexp::nil(), Sexp::int(1)])]);
        let res = env.evaluate(sexp);
        assert_eq!(res, Sexp::int(1));
    }
}
