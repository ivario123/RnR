fn main() {
    let mut b = 0;
    let mut a = &2;
    {
        let b = 2;
        a = &b;
        b;
    };
    *b;
}
