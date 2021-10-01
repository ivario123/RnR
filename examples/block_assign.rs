fn main() {
    let mut a = 6;
    let b = {
        let b = &mut a;
        *b = (*b) + 1;
        *b
    };
    println!("b {}", b)
}
