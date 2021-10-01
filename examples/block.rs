fn main() {
    let mut a = 6;
    let b = {
        a = a + 1;
        a
    };
}
