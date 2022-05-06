use std::thread;
use std::time::Duration;
use std::collections::HashMap;

fn main() {
    let user_value = 10;
    let random_num = 7;


    generate_workout(user_value, random_num);
}

struct Cacher{
    hashmap: HashMap<i32, i32>,
    value: Option<i32>,
}

impl Cacher{
    fn new(value: i32) -> Cacher{
        let mut cacher = Cacher{
            hashmap: HashMap::new(),
            value: Some(value),
        };
        cacher.hashmap.insert(value, value);
        cacher
    }

    fn value(&mut self, value: i32) -> i32{
        if self.hashmap.contains_key(&value){
            return value;
        }
        else{
            self.hashmap.insert(value, value);
        }
        self.hashmap.get(&value).unwrap().clone()
    }
}

fn expensive_closure(num: i32) -> Cacher {
    let mut hashmap: HashMap<i32, i32> = HashMap::new();
    hashmap.insert(num, num);
    println!("Calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    Cacher {
        hashmap: hashmap,
        value: Some(num),
    }
}

fn generate_workout(intensity: i32, random_num: u32){

    let cacher = expensive_closure(intensity);
    if intensity < 25 {
        println!("Today, do {} pushups!", cacher.value.unwrap());
        println!("Next, do {} situps!", cacher.value.unwrap());
    } else {
        if random_num == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                cacher.value.unwrap()
            );
        }
    }
}


#[test]
fn call_with_diffrent_values(){
    let mut cacher = Cacher::new(10);

    let v1 = cacher.value(20);
    let v2 = cacher.value(20);

    assert_eq!(v2, v1);
}

#[test]
fn call(){
    let mut cacher = Cacher::new(10);

    let v1 = cacher.value(2);
    let v2 = cacher.value(4);

    assert_eq!(v2, 4);
}