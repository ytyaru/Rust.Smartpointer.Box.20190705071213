/*
 * Rustのスマートポインタ（Box<T>）。
 * CreatedAt: 2019-07-05
 */
enum List {
    Cons(i32, List),
    Nil,
}
use List::{Cons, Nil};
fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}

