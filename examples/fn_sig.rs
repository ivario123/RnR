fn a(i: i32, bo: bool) -> i32 {
    fn c() -> bool {
        false
    };
    fn b(j: i32) -> i32 {
        a(j, c())
    };

    b(1 + i);
    a(i, bo)
}

fn main() {
    a(1, false);
}
