fn main() {
    let mut a = 0;
    let _b = &mut a;
    let _c = &a;
    // *b = 4;
    // let d = *c; // <- error here, with stacked borrows
}
