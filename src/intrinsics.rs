use crate::ast::{Arg, Block, Expr, Func, Type};
use regex::Regex;
// Implementation of intrinsics for the vm
use crate::ast::Literal;
pub type Intrinsic = fn(Vec<Literal>) -> Literal;

pub fn vm_println() -> (Func, Intrinsic) {
    (
        Func {
            id: Expr::Ident("println!".to_string()),
            args: vec![
                Arg {
                    mutable: false,
                    id: Expr::Ident("str".to_string()),
                    ty: Type::String,
                },
                Arg {
                    mutable: false,
                    id: Expr::Ident("i".to_string()),
                    ty: Type::I32,
                },
            ],
            ty: Type::Unit,
            body: Block {
                statements: vec![],
                semi: false,
            },
        },
        |lit_vec| {
            match &lit_vec[0] {
                Literal::String(s) => {
                    // this regex will find either '{}' or '{:?}'
                    let re = Regex::new(r"\{(:\?)?\}").unwrap();

                    // we split at these points
                    let split = re.split(s);
                    // and collect into vector
                    let vec: Vec<&str> = split.collect();

                    // first print the leading part
                    print!("{}", vec[0]);
                    // then print each matching pair
                    // the value followed by the trailing part
                    for (text, lit) in vec[1..].iter().zip(lit_vec[1..].iter()) {
                        print!("{}{}", lit, text);
                    }

                    println!();
                }
                _ => panic!("ICE - no formatting string in println!"),
            }
            Literal::Unit
        },
    )
}

#[test]
fn regex_test() {
    // this regex will find either '{}' or '{:?}'
    let re = Regex::new(r"\{(:\?)?\}").unwrap();

    // we split at these points
    let split = re.split("a {} b {:?} c");

    // and collect into vector
    let vec: Vec<&str> = split.collect();
    println!("{:?}", vec);
}
