use std::collections::HashMap;

fn main() {
    let list: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 11];

    let midle = list.len() / 2;

    if list.len() % 2 == 0{
        println!("{}", list[midle]);
    }
    else{
        let result:f32 = (list[midle] as f32 + list[midle + 1] as f32) / 2 as f32;
        println!("{}", result);
    }

    let mut map = HashMap::new();

    for num in list{
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }

    let mut master_key: f32 = 0.0;
    let mut master_value = 0;

    for (key, value) in &map{
        if *&value> &master_value{
            master_value = *value;
            master_key = *key as f32;
        }
    }

    println!("{}", master_key);

    let text = "apple";

    let vowel = "aeiou";

    let mut xxt = String::from("aa");

    for word in text.chars(){
        for v in vowel.chars(){
            let count = 0;
            if v == word && count == 0{
                xxt = convert_vowel(text);
                break;
            }
            else{
                xxt = convert_consoant(text);
                break;
            }
        }
        break;

    }

    print!("{}", xxt);




}

fn convert_vowel(text: &str) -> String{
    let result = format!("{}{}", text, "-hay");

    return result;
}

fn convert_consoant(text: &str) -> String{
    let char_vect: Vec<char> = text.chars().collect();

    //let test: String = char_vect[1..];

    let ret = String::from(char_vect[0]);

    let first_word = remove_first(text);

    let result = format!("{}-{}ay", first_word, ret);

    return result;
}

fn remove_first(value: &str) -> &str{
    let mut chars = value.chars();

    chars.next();

    chars.as_str()
}
