fn main() {
    let mut a = 6;
    let _b = {
        a += 1;
        a
    };
}
