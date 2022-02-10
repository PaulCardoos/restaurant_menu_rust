mod item;
mod utils;
use std::io::stdin;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::ErrorKind;
use std::io::BufReader;
use std::io::BufRead;
use std::io::prelude::*;



fn print_greeting(){
    println!("********************\nMenu\n********************\n\n");
}

fn print_options(){
    println!("1. Insert an item\n2. Delete an item\n3. View Menu\n4. Quit");
}

fn main() {

    print_greeting();
    // check for existing items from the file
    let file = match OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open("items.txt"){
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file, {:?}", error),
    };

    // this is where ownership for items exits now
    let items : Vec<String> = utils::retreive_file_contents(&file);
    
    println!("{:?}", items);



    // read file contents into a vector
    loop {
        print_options();
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
