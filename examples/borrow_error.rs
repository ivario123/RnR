fn main() {
    let mut a = 0;
    let b = &mut a;
    let c = &a; // <- Error here with my borrow checker
    *b = 4;
    let _d = *c; // <- error here, with stacked borrows
}
