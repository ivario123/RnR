use rnr::{ast::Prog, common::*, env::Env, type_check::Ty, vm::Val};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "rnr", about = "RNR Rust In Rust - Let's Rock'n Roll")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str))]
    path: PathBuf,

    /// Vm
    #[structopt(short, long)]
    vm: bool,

    /// Type checking
    #[structopt(short, long)]
    type_check: bool,
}

fn main() {
    let opt = Opt::from_args();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&opt.path) {
        Err(why) => panic!("couldn't open {}: {}", opt.path.display(), why),
        Ok(file) => file,
    };

    // reads the file to a string and parses it
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", opt.path.display(), why),
        Ok(_) => {
            print!("rnr input:\n{}", s);
            let ts: proc_macro2::TokenStream = s.parse().unwrap();
            print!("rnr parsing: ");
            let parse: Result<Prog, _> = syn::parse2(ts);
            match parse {
                Err(err) => println!("error: {}", err),
                Ok(prog) => {
                    println!("\nrnr prog:\n{}", prog);

                    if opt.type_check {
                        print!("rnr type checking: ");
                        let mut env: Env<Ty> = Env::new();
                        match prog.eval(&mut env) {
                            Ok(_) => println!("passed"),
                            Err(err) => println!("error: {}", err),
                        }
                    }

                    if opt.vm {
                        println!("rnr evaluating");
                        let mut env: Env<Val> = Env::new();
                        match prog.eval(&mut env) {
                            Ok(_) => println!("rnr evaluating done"),
                            Err(err) => println!("error: {}", err),
                        }
                    }
                }
            }
        }
    }
}
