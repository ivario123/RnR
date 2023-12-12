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

    /// Maximum number of statements to execute
    #[structopt(short, long, default_value = "100")]
    max_iter: usize,
}

fn main() {
    // Set a custom panic hook
    std::panic::set_hook(Box::new(|panic_info| {
        let _ = write!(std::io::stderr(), "Panic: {}", panic_info);
    }));
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
        eprintln!("error: {}", parse.err().unwrap());
        return;
    }
    let prog = parse.unwrap();
    println!("\nrnr prog:\n{}", prog);
    let mut type_check_passed = true;
    if opt.type_check {
        print!("rnr type checking: ");
        match check!(prog) {
            Ok(_) => println!("passed"),
            Err(err) => {
                type_check_passed = false;
                eprintln!("error: {}", err)
            }
        }
    }

    if opt.vm && type_check_passed {
        println!("rnr evaluating");
        let iter = opt.max_iter;
        match eval!(prog, iter) {
            Ok(_) => println!("rnr evaluating done"),
            Err(err) => eprintln!("error: {}", err),
        }
    }
}
