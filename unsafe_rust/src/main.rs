use std::slice;

fn main() {
    let mut num = 5;
    
    let r1 = &num as *const i32;

    let r2 = &mut num as *mut i32;

    let address = 0x012345usize;

    let r = address as *const i32;

    unsafe{
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);    
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = split_as_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);


    fn split_as_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]){
        let len = slice.len();
        let ptr = slice.as_mut_ptr();

        assert!(mid <= len);

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    
    for x in (a, b).1{
        println!("{}", x);
    }

    println!("======================================================");

    unsafe {
        println!("Absolulte value of -3 according to C: {}", abs(-3));
    }
}


extern "C" {
    fn abs(input: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn call_from_c(){
    println!("Just called a Rust function from C!");
}

