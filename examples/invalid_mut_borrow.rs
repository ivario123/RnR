fn main() {
    let a = 0;
    let mut b = &&a;
    let c = &mut b;
    *c = 2;
    println!("{}", b);
}
