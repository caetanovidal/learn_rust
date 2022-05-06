use std::fmt;


fn main() {

    let person = Human;

    person.fly();
    Wizard::fly(&person);
    Pilot::fly(&person);

    println!("==========================================================");

    let xxt: Point = { Point {x: 2, y: 3}};

    println!("{}", xxt);

    xxt.outline_print();
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking. ")
    }
}

impl Wizard for Human {
    fn fly(&self){
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait OutlinePrint: fmt::Display{
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));        
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}