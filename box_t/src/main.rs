use crate::List::{Cons, Nil};
use std::ops::Deref;
use std::mem::drop;

fn main() {
    let b = Box::new(5); // storing in the heap memory
    println!("b = {}", b);

    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;

    let y = Mybox::new(x);

    assert_eq!(5, x);

    assert_eq!(5, *y);

    let c  = CustomSmartPointer {
        data: String::from("My stuff")
    };

    drop(c);

    let d  = CustomSmartPointer {
        data: String::from("Other Stuff")
    };

    println!("CustomStartPointer Created")

}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct Mybox<T>(T);

impl<T> Mybox<T> {
    fn new(x: T) -> Mybox<T> {
        Mybox(x)
    }
}

impl<T> Deref for Mybox<T>{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self){
        println!("Dropping customSmartPointer with data `{}`!", self.data)
    }
}