use std::io;


fn f_to_c (f:f32) -> f32 {
    (f-32.0)*(0.5556)
}
fn c_to_f (c:f32) -> f32{
    (c/0.5556)+32.0
}

fn main() {
    println!("Welcome to the temperature converter!");
    
    let temp: f32 = loop {
        println!("Please input the temperature you would like to convert. We will choose the temperature type next.");
        let mut inp = String::new();
        io::stdin().read_line(&mut inp).expect("Failed to read line");
        
        let inp: f32 = match inp.trim().parse() {
            Ok(x) => x,
            Err(_) => {
                println!("This is not a valid temperature! Try again...");
                continue;
            }
        };

        break inp;
        

 
    };
    
    println!("Your chosen temperature is: {}", temp);


    let choice = loop {
        println!("Please input the temp type we are converting to:\n'F' for fahrenheit\n'C' for celsius.");
        let mut inp = String::new();
        io::stdin().read_line(&mut inp).expect("Failed to read line");
        
        let inp: char = match inp.trim().parse() {
            Ok(x) => x,
            Err(_) => {
                println!("This is not a char! Try again...");
                continue;
            }
        };

        match inp {
            'F' | 'C' => break inp,
            _ => {
                println!("Not a valid choice");
                continue;
            }
        };


        
    };
    
    match choice {
        'F' => println!("Your converted temp is {} celsius!", c_to_f(temp)),
        'C' => println!("Your converted temp is {} fahrenheit!", f_to_c(temp)),
        _ => println!("Error has been encountered!")
    }
}
