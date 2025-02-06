
use std::{path::Display, rc::Rc};

use List::{Cons , Nil}; 



enum List {

    Cons(i32, Rc<List>),
    Nil,

}


struct CustomSmartPointers {



}


fn main() {

    let a = Cons(5, Rc::new(Cons(10, Rc::new(Nil)))); 
    let b = Cons(3, Rc::new(a));





    let x :i32 = 5;

    let list = Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil))))));


}
