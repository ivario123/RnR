fn tmp() {}

fn main() {
    let a = &1;
    let a = &mut { *a + 5 };
    *a += 1;
    println!("{}", *a);
}
fn other_tmp() {}
