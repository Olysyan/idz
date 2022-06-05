use std::fs;
use std::process::Command;

pub fn first(from:&str,to:&str)->std::io::Result<()>{
    fs::copy(from,to)?;
    Ok(())
}

pub fn copy_directory(from:&str,to:&str) {
    let path_to= fs::create_dir(to);

    if path_to.is_err(){panic!("Папка уже существует!")}

    let paths_from = fs::read_dir(from).unwrap();

    for path in paths_from {
    let path_new = path.unwrap().path();
    println!("{} {}",to.to_owned() +"/"+ &path_new.file_name().unwrap().to_str().unwrap(), (&path_new.to_str().unwrap()).to_string());
    let output = 
        Command::new("./target/debug/first.exe")
                .args([(&path_new.to_str().unwrap()).to_string(), to.to_owned() +"/"+ &path_new.file_name().unwrap().to_str().unwrap() ])
                .output()
                .expect("failed to execute process");
    

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);

        print!("rustc завершился успешно и вывел в stdout:\n{}", s);
    } else {
        copy_directory(((&path_new.to_str().unwrap()).to_string()).as_str(), (to.to_owned() +"/"+ &path_new.file_name().unwrap().to_str().unwrap()).as_str());
    }
}
}
