use std::path::Path;
use std::fs::File;

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
    let input_filename = Path::new(_filename);
    

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
