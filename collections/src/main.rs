use std::collections::HashMap;
fn main() {
    println!("Hello, world!");

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];

    println!("The third is {}", third);

    match v.get(3){
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third best"),
    }

    match &v[1]{
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Panic"),
    }

    let mut v = vec![100, 32, 57];

    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Red"), 10);
    scores.insert(String::from("Green"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(50);

    println!("{:?}", scores);

    let text = "hello world wonderful world world wonderful pica";

    let mut map = HashMap::new();

    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}


