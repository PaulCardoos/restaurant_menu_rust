use std::io::stdin;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::prelude::*;
use std::vec::Vec;



pub fn retreive_file_contents(file:&File) -> Vec<String> {
    let reader = BufReader::new(file);
    let mut items : Vec<String> = Vec::new();
   
    for line in reader.lines(){
        match &line {
            Ok(l) => {
                let s = String::from(l);            
                items.push(s)
                
            },
            Err(_) => panic!("Idk what error"),
        };

    }

    items


    
}