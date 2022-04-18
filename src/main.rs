use std::{
    fs::File,
    io::{stdin, stdout, Write, Read},
    path::Path,
    env::current_dir,
};
use sha256::digest_bytes;

fn main() {
    println!("Current working directory is {}", current_dir().unwrap().to_string_lossy());

    let mut s = String::new();

    println!("Please enter the path to the file whose hash you want to find out: ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    
    let path = Path::new(s.as_str());
    
    let mut file = File::open(&path).expect("Couldn't open the file");

    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf).expect("Couldn't read the file");
    
    let hash = digest_bytes(buf.as_slice());

    println!("Hash of \"{}\" is {}", path.to_string_lossy(), hash);
}
