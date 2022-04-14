fn main() {

    {                      // s is not valid here, it’s not yet declared
        let _s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid
    
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`


    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {} s2 = {}", s1, s2);

    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);            

    let x = 5;      
                   
    makes_copy(x);
                                    
    println!("{}", x)                               
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.