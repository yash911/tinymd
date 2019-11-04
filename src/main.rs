use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader}; //BufReader => Let's us buffer a file into the memory
                                   //BufRead => Let's us read those buffers line by line
fn get_title() -> String {
    let mut title = String::from(env!("CARGO_PKG_NAME"));
    title.push_str(" (v");
    title.push_str(env!("CARGO_PKG_VERSION"));
    title.push_str("), ");
    title.push_str(env!("CARGO_PKG_DESCRIPTION"));
    title
}

fn parse_markdown_file(_filename: &str) {
    print_short_banner();
    println!("[ INFO ] Parser firing up!");
    //Create a path variable from the filename
    let input_filename = Path::new(_filename);

    //Try to open the file using the File
    //varible by initializing it with the Path variable
    let file = File::open(&input_filename)
                .expect("[ ERROR ] Failed to open file!");

    //Setting up state variable to keep track of header and paragraph tags
    let mut _htag: bool = false;
    let mut _ptag: bool = false;

    //Vector is made mutable because we want to add tokens as we parse(read go)
    let mut tokens: Vec<String> = Vec::new();

    //Using BufReader for storing the file into a buffer(memory)
    let reader = BufReader::new(file);

    //Looping through the reader lines and unwrapping as we go
    for line in reader.lines() {

    }





}

fn print_short_banner() {
    println!("{}", get_title());

}

fn print_long_banner() {
    print_short_banner();
    let author = String::from(env!("CARGO_PKG_AUTHORS"));
    let homepage = String::from(env!("CARGO_PKG_HOMEPAGE"));
    println!("Written by: {}\nHomepage: {}\nUsage: tinymd <somefile>.md",
             author,
             homepage
            );

}

fn usage() {
    print_long_banner();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        2 => parse_markdown_file(&args[1]),
        _ => {
            println!("[ ERROR ] Invalid invocation! ");
            usage();
        }
    }
}
