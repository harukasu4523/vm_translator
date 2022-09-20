mod translate;


use std::env;
use std::fs::File;
use std::process;
use std::io::{BufRead, BufReader, Error, Read, Write};
use std::iter::Map;

fn main() {

    translate::translate();

    // let mut output_file = create_file(&args[1]);
    // init つどつどファイルに書き込むかbufferにいれて全部書き込むタイプにするか。。。
    // cpu_init_set(&mut output_file); 

    // let mut asm = String::new();
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

// fn get_file_contents (path: &String) -> Result<String, Error>{
//     let mut f = File::open(&path)?;
//     let mut r = String::new();
//     f.read_to_string(&mut r)?;
//     Ok(r)
// }

