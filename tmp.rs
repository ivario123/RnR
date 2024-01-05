fn main() {
    let mut a = 2;
    let b = &&mut a;
    **a = 2;
}
