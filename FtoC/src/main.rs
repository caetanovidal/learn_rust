use std::io;


fn main() {

    loop{

        println!("To convert F to C press 0\nTo convert C to F press 1");

        let mut choice = String::new();
        
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        
        if choice.trim() == "1"{
            println!("Celsius: ");
        }else{
            println!("Fahren: ");
        }

        let mut temp = String::new();
        
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");
        
        let temp: f64 = match temp.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        let temp_f: f64;

        if choice.trim() == "1"{
            temp_f = temp * 1.8 + 32 as f64;
        }
        else{
            temp_f = (temp - 32 as f64)/1.8;
        }
        

        println!("converted temp = {}", temp_f)
    }
    
}
