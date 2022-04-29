use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


fn main() {
    let path = Path::new("data/image.jpeg");
    let display = path.display();
    let meta = std::fs::metadata(path).unwrap();
    println!("Created     {:?}", meta.created().unwrap().elapsed());
    println!("Accessed    {:?}", meta.accessed().unwrap().elapsed());
    println!("Modified    {:?}", meta.modified().unwrap().elapsed());
    println!("File type   {:?}", meta.file_type());
    println!("Permissions {:?}", meta.permissions());

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let hash = md5::compute(buffer);
    println!("{:?}", hash);

    // Copying
    let copy_path = Path::new("data/image2.jpeg");
    std::fs::copy(path, copy_path).unwrap();
    
    // Testing if md5 is the same
    let mut file2 = File::open(&copy_path).unwrap();
    let mut buffer2 = Vec::new();
    file2.read_to_end(&mut buffer2).unwrap();
    let hash2 = md5::compute(buffer2);
    println!("{:?}", hash2);

    let meta = std::fs::metadata(copy_path).unwrap();
    println!("Created     {:?}", meta.created().unwrap().elapsed());
    println!("Accessed    {:?}", meta.accessed().unwrap().elapsed());
    println!("Modified    {:?}", meta.modified().unwrap().elapsed());
    println!("File type   {:?}", meta.file_type());
    println!("Permissions {:?}", meta.permissions());

    // Modifying some bytes
    let copy_path = Path::new("data/image2.jpeg");
    let mut file3 = File::open(&copy_path).unwrap();
    let mut buffer3 = Vec::new();
    file3.read_to_end(&mut buffer3).unwrap();

    let n = buffer3.len();
    buffer3[n-10] = 1;
            
    let edit_path = Path::new("data/image3.jpeg");
    let mut new_file = File::create(edit_path).unwrap();
    new_file.write_all(&buffer3).unwrap();
    let hash3 = md5::compute(buffer3);
    println!("{:?}", hash3);


}