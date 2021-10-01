fn main() {
    fn a() -> i32 {
        let a: i32 = 1 + 2; // a == 3
        let mut a: i32 = 2 + a; // a == 5
        if true {
            a = a - 1; // outer a == 4
            let mut a: i32 = 0; // inner a == 0

            a = a + 1 // inner a == 1
        } else {
            a = a - 1
        };
        a // a == 4
    }
}
