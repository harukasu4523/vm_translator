pub mod parser;

use crate::parser::*;
use std::env;
use std::fs::File;
use std::process;
use std::io::{Error, Read};

pub enum CommandType<'a> {
    ARITHEMETIC(&'a str),
    PUSH(&'a str, &'a str),
    POP(&'a str, &'a str),
    COMMENT(&'a str)
    // C_LABEL,
    // C_GOTO,
    // C_IF,
    // C_FUNCTION,
    // C_RETURN,
    // C_CALL
}

pub fn translate(){
    let args:Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        println!("Error: Not enough arguments");
        process::exit(1);
    }

    let mut contents = String::new();
    if args[1].ends_with(".vm"){
        match get_file_contents(&args[1]) {
            Ok(r) => {
                println!("Ok: File Open");
                // parse function
                contents = r;

            }
            Err(e) => {
                println!("Error:{}", e);
                process::exit(1);
            }
        }
    }else {println!(".vm file じゃないよ")}//open directry

        
    let lines: Vec<String> = contents.lines().map(str::trim)
        .map(|l| match l.find("//"){
            Some(v) => l[0..v].to_string(),
            None => l.to_string(),
        })
        .collect();
    println!("{:?}", lines);

    let commands:Vec<CommandType> = lines.iter().map(|l| parse_command(&l)).collect();
    
}



fn get_file_contents (path: &String) -> Result<String, Error>{
    let mut f = File::open(&path)?;
    let mut r = String::new();
    f.read_to_string(&mut r)?;
    Ok(r)
}
// }