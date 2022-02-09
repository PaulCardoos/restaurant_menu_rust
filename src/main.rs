mod item;
use std::io::stdin;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::ErrorKind;

fn main() {

    // check for existing items from the file
    let file = match OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open("items.txt"){
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file, {:?}", error),
    };

    // read file contents into a vector
    
 
    
    loop {
        let mut user_choice = String::new();
        stdin().read_line(&mut user_choice).expect("Could not read line");
        let num : u8 = match user_choice.trim().parse(){
            Ok(num) => num,
            Err(err) => panic!("Did understand, {:?}", err),
        };

        match &num {
            0 => println!("Enter a valid choice"),
            1 => println!("God"), 
            2 => println!("Loves"),
            3 => println!("Me"),
            4 => break,
            5_u8..=u8::MAX => println!("Enter a valid choice")
        }
    }

    

    

}
