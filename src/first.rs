use std::env;

fn main()->std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    second::first(&args[1].as_str(),&args[2].as_str())
}