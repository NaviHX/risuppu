use risuppu::semantic::Env;
use risuppu::sexp::parse::parse_sexp;

use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

fn main() {
    let mut env = Env::new();
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
