fn main() {
    let mut a = 0;
    let b = &mut a;
    let c = &a;
    // *b = 4;
    // let d = *c; // <- error here, with stacked borrows
}
