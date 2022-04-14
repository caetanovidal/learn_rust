fn main() {
    let st = String::from("guys        guys");
    let word = first_word(&st);
    println!("{}", word);
}


fn first_word(s: &str) -> &str{

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
