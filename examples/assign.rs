fn a() -> i32 {
    2
}
fn main() {
    let a = &mut { 125 };
    {
        let b = &mut *a;
        *b = *b + 1;
    };
    let mut a: i32 = 5 + 2;
    a += 1;
    let a = (5 + 1) * 6 + 6;
    println!("{}", a);
}
