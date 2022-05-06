fn main() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    while let Some(top) = stack.pop() {
        println!("{}", top)
    }

    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("matche, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {

        Message::ChangeColor(Color::Rgb(r, g, b)) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    
    }

    let x = 6;
    let y = true;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),

    }
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}
    