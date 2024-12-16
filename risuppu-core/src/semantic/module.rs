use std::path::PathBuf;

use crate::sexp::{Ptr, Sexp};

use super::Env;

pub fn process_provide(body: Ptr<Sexp>, env: &mut Env) -> Ptr<Sexp> {
    let ident = body.car();
    let provided = body.cdr().car();
    let provided = env.evaluate(provided);
    env.add_provided(ident, provided);

    Sexp::nil()
}

pub fn process_require(body: Ptr<Sexp>, env: &mut Env) -> Ptr<Sexp> {
    // TODO: Clone cost
    let mut required_env = env.clone();
    let module_name = body.car();
    let prefix = body.cdr().car();

    if let Sexp::SString(module) = module_name.as_ref() {
        match std::fs::read_to_string(module) {
            Ok(content) => {
                let buf = content.as_str();
                evaluate_buf(buf, &mut required_env);
            }
            Err(e) => {
                println!("Error when requiring '{module}': {e}");
                return Sexp::nil();
            }
        }
    }

    if let Sexp::Identifier(module) = module_name.as_ref() {
        let mut module_path: PathBuf = std::env::var("RISP_LIB")
            .expect("Cannot found $RISP_LIB. Please set the environment var before running")
            .into();
        module_path.push(format!("{module}.risp"));
        match std::fs::read_to_string(module_path.clone()) {
            Ok(content) => {
                let buf = content.as_str();
                evaluate_buf(buf, &mut required_env);
            }
            Err(e) => {
                println!("Error when requiring '{}': {e}", module_path.to_str().unwrap());
                return Sexp::nil();
            }
        }
    }

    let prefix = if let Sexp::SString(s) | Sexp::Identifier(s) = prefix.as_ref() {
        s.as_str()
    } else {
        ""
    };
    for (k, v) in required_env.get_provided().into_iter() {
        let import_ident = Sexp::identifier(format!("{prefix}{k}"));
        env.set_global(import_ident.clone(), v.clone());
        env.add_provided(import_ident, v);
    }

    Sexp::nil()
}

fn evaluate_buf(mut buf: &str, env: &mut Env) {
    while !buf.is_empty() {
        match crate::sexp::parse::parse_sexp(buf) {
            Ok((remaining_buf, s)) => {
                env.evaluate(s);
                buf = remaining_buf;
            }
            Err(e) => {
                println!("Error when parsing: {e}");
                break;
            }
        }
    }
}
