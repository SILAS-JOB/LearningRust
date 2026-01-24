//COns List ds
// enum List {
//     Cons(i32, List),
//     Nil,
// }
//
// use crate::List::{Cons, Nil};
//
// fn main() {
//     let list = Cons(1, Cons(2, Cons(3, Nil)));
//     let x = Box::new(5);
// }
//
//

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    let x = Box::new(5);
}
