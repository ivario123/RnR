fn main() {
    let mut a = 2;
    let b = &mut &mut a;
    **b = 3;
    println!("{}", **b);
}
