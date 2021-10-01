fn main() {
    let mut a = 6;
    {
        let b = &mut a;
        *b = 2;
    }
    println!("a {}", a);
}
