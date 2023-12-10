use rnr::prelude::*;
use rnr::{check, eval, parse};
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
    if let Err(why) = file.read_to_string(&mut s) {
        panic!("couldn't read {}: {}", opt.path.display(), why)
    }

    print!("rnr input:\n{}", s);
    let ts: proc_macro2::TokenStream = s.parse().unwrap();
    print!("rnr parsing: ");
    let parse: Result<rnr::ast::Prog, _> = parse!(ts);
    if parse.is_err() {
        println!("error: {}", parse.err().unwrap());
        return;
    }
    let prog = parse.unwrap();
    println!("\nrnr prog:\n{}", prog);

    if opt.type_check {
        print!("rnr type checking: ");
        match check!(prog) {
            Ok(_) => println!("passed"),
            Err(err) => println!("error: {}", err),
        }
    }

    if opt.vm {
        println!("rnr evaluating");
        match eval!(prog) {
            Ok(_) => println!("rnr evaluating done"),
            Err(err) => println!("error: {}", err),
        }
    }
}
