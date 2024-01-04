fn main() {
    fn a() {
        fn a(_i: i32) {
            a(1);
        };

        fn b() {
            a(1);
            c()
        };
    };

    fn c() {}
}
