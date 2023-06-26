#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
    println!("{:?}", b);
    println!("{:?}", c);
    let a = Rc::new(1);
    let b = Rc::clone(&a); // Makes a shallow copy of A
    println!("{}", b);
}
