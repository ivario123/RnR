fn main() {
    let a = 0;
    let b = &mut a;
    *b = 2;
    println!("{}", b);
}
