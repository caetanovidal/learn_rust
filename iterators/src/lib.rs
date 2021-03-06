#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe>{
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_by_size(){
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("Sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("Sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("Boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(in_my_size, vec![
            Shoe {
                size: 10,
                style: String::from("Sneaker"),
            },
            Shoe {
                size: 10,
                style: String::from("Boot"),
            },
        ]);
    }
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter {count: 0}
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item>{
        if self.count < 5 {
            self.count += 1
            Some(self.count)
        }
        else{
            None
        }
    }
}

#[test]
fn callling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(Counter.next(), Some(1));
    assert_eq!(Counter.next(), Some(2));
    assert_eq!(Counter.next(), Some(3));
    assert_eq!(Counter.next(), Some(4));
    assert_eq!(Counter.next(), Some(5));
    assert_eq!(Counter.next(), None);
}