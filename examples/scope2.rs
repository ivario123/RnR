fn main() {
    fn f(a: i32, b: i32) {
        println!("{}", a);
        println!("{}", b);
    }

    let mut a = 0;
    f(
        {
            a = a + 1;
            a
        },
        {
            a = a + 2;
            a
        },
    );
}
