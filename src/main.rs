fn main(){
    usage();
}

fn usage(){
    let version = env!("CARGO_PKG_VERSION");
    println!("tinymd, a markdown compiler written by yash911");
    println!("Version {}", version);
}

