use rnr::codegen::CompileTarget;
use rnr::{check, eval};
use rnr::{prelude::*};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use structopt::StructOpt;
extern crate strip_ansi_escapes;
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

    #[structopt(short, long)]
    target: Option<CompileTarget>,

    #[structopt(short, long, default_value = "")]
    output_file: String,
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
    print!("rnr parsing: ");
    let prog: Ast<Prog> = s.into();
    println!("\nrnr prog:\n{}", prog);
    let mut type_check_passed = true;
    if opt.type_check {
        print!("rnr type checking: ");
        match check!(prog) {
            Ok(_) => println!("passed"),
            Err(err) => {
                type_check_passed = false;
                eprintln!("error: {}", err);
                return;
            }
        }
    }

    if opt.vm && type_check_passed {
        println!("rnr evaluating");
        let iter = opt.max_iter;
        match eval!(prog, iter) {
            Ok(_) => println!("rnr evaluating done"),
            Err(err) => {
                eprintln!("error: {}", err);
                return;
            }
        }
    }

    if opt.target.is_none() {
        return;
    }

    let _target = opt.target.unwrap();
    let output = opt.output_file;
    let asm = prog.codegen();
    println!("{asm}");
    let mut file = match File::options()
        .write(true)
        .create(true)
        .open(output.clone())
    {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Cannot open file {output} in write mode, error {e}");
            return;
        }
    };
    let plain_bytes = strip_ansi_escapes::strip(format!("{asm}").as_bytes());
    file.write(&plain_bytes).unwrap();
}
