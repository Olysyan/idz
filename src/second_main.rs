use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len()!=3{panic!("неверное колличество аргументов!")}
    
    second::copy_directory(&args[1].as_str(),&args[2].as_str());
    
}

