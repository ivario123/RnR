use crate::{env::Env, env::Ref};

use crate::error::Error;

pub trait Eval<T>
where
    T: Clone,
{
    fn eval(&self, env: &mut Env<T>) -> Result<(T, Option<Ref>), Error>
    where
        T: Clone;
}

pub fn parse<T1, T2>(s: &str) -> (T1, Env<T2>)
where
    T1: syn::parse::Parse + std::fmt::Display,
    T2: Clone,
{
    let ts: proc_macro2::TokenStream = s.parse().unwrap();
    let r: T1 = syn::parse2(ts).unwrap();
    println!("{}", r);
    (r, Env::new())
}

pub fn parse_test<T1, T2>(s: &str) -> Result<T2, Error>
where
    T1: syn::parse::Parse + std::fmt::Display + Eval<T2>,
    T2: std::fmt::Debug + Clone,
{
    let (bl, mut env) = parse::<T1, T2>(s);
    let (v, _) = bl.eval(&mut env)?;
    println!("\nreturn {:?}", v);
    Ok(v)
}
