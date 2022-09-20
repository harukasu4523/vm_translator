// use std::fs::File;
// use std::process;
// use std::io::{BufRead, BufReader, Error, Read, Write};
// use std::iter::Map;

// pub mod parse{

pub fn parse_process(){

let commands:Vec<CommandType> = lines.iter().map(|l| parse_command(&l)).collect();
// cordwriter
for command in commands {
    let line = match command {
        CommandType::COMMENT(_) => continue,
        CommandType::ARITHEMETIC(cmd) => println!("{}",cmd),
        CommandType::POP(segment, num) => segment_sec(segment, num) , //println!("segment:{} num{}",segment, num)
        CommandType::PUSH(segment, num) => println!("segment:{} num{}",segment, num)
    };
    if let Some(line) = line {
        println!("{}",line);
    }
}

fn parse_command(line: &str)->CommandType{
    let elements:Vec<&str> = line.split_whitespace().collect();
    if elements.len() <= 0{
        return CommandType::COMMENT("empty")
    }
    match elements[0] {
        "push" => CommandType::PUSH(elements[1], elements[2]),
        "pop" => CommandType::POP(elements[1], elements[2]),
        "add" => CommandType::ARITHEMETIC(elements[0]),
        "sub" => CommandType::ARITHEMETIC(elements[0]),
        "neg" => CommandType::ARITHEMETIC(elements[0]),
        "eq" => CommandType::ARITHEMETIC(elements[0]),
        "gt" => CommandType::ARITHEMETIC(elements[0]),
        "lt" => CommandType::ARITHEMETIC(elements[0]),
        "and" => CommandType::ARITHEMETIC(elements[0]),
        "or" => CommandType::ARITHEMETIC(elements[0]),
        "not" => CommandType::ARITHEMETIC(elements[0]),
        //"lavel" => CommandType::ARITHEMETIC(), 何返すかわからん
        _ => {
            println!("Invaild command {}",line);
            process::exit(1);
        }
    }   
}

    }
    // fn parse_command(line: &str)->CommandType{
    //     let elements:Vec<&str> = line.split_whitespace().collect();
    //     if elements.len() <= 0{
    //         return CommandType::COMMENT("empty")
    //     }
    //     match elements[0] {
    //         "push" => CommandType::PUSH(elements[1], elements[2]),
    //         "pop" => CommandType::POP(elements[1], elements[2]),
    //         "add" => CommandType::ARITHEMETIC(elements[0]),
    //         "sub" => CommandType::ARITHEMETIC(elements[0]),
    //         "neg" => CommandType::ARITHEMETIC(elements[0]),
    //         "eq" => CommandType::ARITHEMETIC(elements[0]),
    //         "gt" => CommandType::ARITHEMETIC(elements[0]),
    //         "lt" => CommandType::ARITHEMETIC(elements[0]),
    //         "and" => CommandType::ARITHEMETIC(elements[0]),
    //         "or" => CommandType::ARITHEMETIC(elements[0]),
    //         "not" => CommandType::ARITHEMETIC(elements[0]),
    //         //"lavel" => CommandType::ARITHEMETIC(), 何返すかわからん
    //         _ => {
    //             println!("Invaild command {}",line);
    //             process::exit(1);
    //         }
    //     }   
    // }

    // fn get_file_contents (path: &String) -> Result<String, Error>{
    //     let mut f = File::open(&path)?;
    //     let mut r = String::new();
    //     f.read_to_string(&mut r)?;
    //     Ok(r)
    // }
    
// }