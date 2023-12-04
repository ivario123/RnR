fn main() {
    let mut a = 6;
    let b = {
        let b = &mut a;
        *b += 1;
        *b
    };
    println!("b {}", b)
}
