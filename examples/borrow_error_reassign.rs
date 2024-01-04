fn main() {
    let mut a = 2;
    let b = &mut a;
    let a = 1;
    a;
    *b = 1;

    let mut b = &a;
    {
        let a = 2;
        b = &a;
    };
    *b;
}
