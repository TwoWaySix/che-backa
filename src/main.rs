use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


fn main() {
    // Create a path to the desired file
    let path = Path::new("data/image.jpeg");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let hash = md5::compute(buffer);
    println!("{:?}", hash);

    let copy_path = Path::new("data/image2.jpeg");
    std::fs::copy(path, copy_path).unwrap();
    
    // Testing if md5 is the same
    let mut file2 = File::open(&copy_path).unwrap();
    let mut buffer2 = Vec::new();
    file2.read_to_end(&mut buffer2).unwrap();
    let hash2 = md5::compute(buffer2);
    println!("{:?}", hash2);

}