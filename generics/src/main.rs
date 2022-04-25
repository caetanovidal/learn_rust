fn main() {

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let string1 = String::from("long string is long");
    let string3 = "a";
    let string2 = "abcde";

    
    let string3 = longest(&string1, &string2);
    


    println!("The longest string is {}", string3);
}

fn largest<T>(list: &[T]) -> &T  
    where T: PartialOrd
{
    let mut largest = &list[0];

    for item in list {
        if item > largest{
            largest = &item;
        }
    }

    largest
}

//fn largest<T>(list: &[T]) -> T where T: PartialOrd + Clone{
//   let mut largest = list[0].clone();
//
//    for item in list {
//        if item > &largest{
//            largest = item.clone();
//        }
//    }
//
//    largest
//}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

