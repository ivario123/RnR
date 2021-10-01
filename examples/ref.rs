fn main() {
    let a = &1;
    let mut a = &mut { *a + 5 };
    *a = *a + 1;
    println!("{}", *a);
}
