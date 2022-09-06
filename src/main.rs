use std::env;
use std::fs::File;
use std::process;
use std::io::{BufRead, BufReader, Error, Read, Write};
use std::iter::Map;

enum command_type {
    C_ARITHEMETIC,
    C_PUSH,
    C_POP,
    C_LABEL,
    C_GOTO,
    C_IF,
    C_FUNCTION,
    C_RETURN,
    C_CALL
}


fn main() {
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
    let commands: Vec<String> = contents
        .lines()
        .map(str::trim)
        .map(|l| match l.find("//"){
            Some(v) => l[0..v].to_string(),
            None => l.to_string(),
        })
        .collect();
    println!("{:?}", commands);

    // let mut output_file = create_file(&args[1]);
    // init つどつどファイルに書き込むかbufferにいれて全部書き込むタイプにするか。。。
    // cpu_init_set(&mut output_file); 

    let mut asm = String::new();

    
    // parse
    for command in commands {
        if command.len() == 0 {
            continue;
        }
        let elements:Vec<&str> = command.split_whitespace().collect();
        match elements[0] {
            "push" => 

        }
        println!("{:?}",elements);
    }

    
}

fn cpu_init_set(f: &mut File) {
    // write
    write!(f, "// init stack point
@256
D=A
@SP
M=D
").expect("write error");
}

fn create_file(path: &String) -> File {
    if path.ends_with(".vm"){
        let split_path:Vec<&str> = path.rsplitn(2, '.').collect();
        let out_path = format!("{}.asm",split_path[1]);
        let f = File::create(out_path).expect("failed open file");
        f
    }else {
        println!("ディレクトリだよ！");
        process::exit(1);
    }
}

fn get_file_contents (path: &String) -> Result<String, Error>{
    let mut f = File::open(&path)?;
    let mut r = String::new();
    f.read_to_string(&mut r)?;
    Ok(r)
}

