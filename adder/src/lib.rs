#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            _width: 8,
            _height: 7,
        };
        let smaller = Rectangle {
            _width: 5,
            _height: 1,
        };

        assert!(larger._can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            _width: 8,
            _height: 7,
        };
        let smaller = Rectangle {
            _width: 5,
            _height: 1,
        };

        assert!(!smaller._can_hold(&larger))
    }
    
    #[test]
    fn it_adds_two(){
        assert_eq!(4, add_two(2),
        "should add two numbers");
    }

    #[test]
    fn it_works() -> Result<(), String>{
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    _width: u32,
    _height: u32,
}

impl Rectangle {
    fn _can_hold(&self, other: &Rectangle) -> bool {
        self._width > other._width && self._height > other._height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}