fn main() {
    let mut a = 0;
    let b = &mut a;
    *b = 2;
    println!("{}", a);
}
