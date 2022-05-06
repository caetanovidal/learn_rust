fn main() {
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("Hello, world!"));

    fn takes_long_type(f: Thunk){}

    fn return_long_type() -> Thunk{
        Box::new(|| println!("Hello, world!"))
    }

    fn bar() -> ! {
        return_long_type();
        panic!("This should not be printed!");
    }

    //bar();

    fn add_one(x: i32) -> i32 {
        x + 1
    }

    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
    
}
