fn main() {
    let a = 1;
    a;
    let mut a_2 = 2;
    {
    let b = &mut a_2;
    b;
    };
    let c = &a_2;
    c;
    *c;
}
