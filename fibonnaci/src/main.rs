fn main() {

    let mut first = 0;
    let mut second = 1;
    let mut hold = 0;

    while first < 8000{
        println!("{}", second + first);
        hold = second;
        second = second + first;
        first = hold;
    }
}
