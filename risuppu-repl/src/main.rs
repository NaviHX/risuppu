use std::fs::File;
use std::io::Read;
use std::path::Path;

use risuppu::semantic::Env;
use risuppu::sexp::parse::parse_sexp;

use risuppu_std::base::base as load_base;
use risuppu_std::string::string as load_string;
use risuppu_std::arithmetic::arithmetic as load_arithmetic;

use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

mod arg;
use arg::Arg;

use clap::Parser;

fn evaluate_file(file: &Path, env: &mut Env) -> std::io::Result<()> {
    let mut file = File::open(file)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut remaining_content = content.as_str();
    while !remaining_content.is_empty() {
        match parse_sexp(remaining_content) {
            Ok((unparsed, sexp)) => {
                remaining_content = unparsed;
                env.evaluate(sexp);
            }
            Err(e) => {
                println!("{e}");
                break;
            }
        }
    }

    Ok(())
}

fn main() {
    let arg: Arg = Arg::parse();

    let mut env = Env::new();
    load_base(&mut env);
    load_string(&mut env);
    load_arithmetic(&mut env);

    for file in arg.files {
        if let Err(e) = evaluate_file(&file, &mut env) {
            println!("Error when evaluating {}: {}", file.to_string_lossy(), e);
        }
    }

    #[cfg(not(debug_assertions))]
    if !arg.interact {
        return;
    }

    let mut rl = DefaultEditor::new().expect("Cannot read line!");

    loop {
        let readline = rl.readline("Risuppu >> ");
        match readline {
            Ok(line) => {
                let parse_result = parse_sexp(line.as_str());
                match parse_result {
                    Ok((_, sexp)) => {
                        let eval = env.evaluate(sexp);
                        println!("> {eval}");
                    }
                    Err(e) => {
                        println!("{e}");
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
