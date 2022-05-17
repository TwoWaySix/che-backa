use std::fs::File;
use std::fs;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use chrono::Utc;

struct Config {
    backup_src: PathBuf,
    backup_dest: PathBuf,
    dt_backup_hours: f64
}

fn main() {
    // TODO: Get these values from command line args
    let backup_src = PathBuf::from("./data/data");
    let backup_dest = PathBuf::from("./data/backups");
    let dt_backup_hours = 0.01;

    let cfg = Config{
        backup_src: backup_src,
        backup_dest: backup_dest,
        dt_backup_hours: dt_backup_hours
    };
    
    start_program_loop(&cfg);
    
}

fn start_program_loop(cfg: &Config) {
    loop {
        create_backup(cfg);
        
        let dur = Duration::from_secs(
            (cfg.dt_backup_hours * 3600.) as u64
        );
        std::thread::sleep(dur);
    }
}

fn create_backup(cfg: &Config) {
    let new_backup_directory = create_new_backup_directory(cfg);
        
    recursively_iterate_files(&cfg.backup_src, &new_backup_directory);
}

fn create_new_backup_directory(cfg: &Config) -> PathBuf {
    let now = Utc::now();
    let new_dir_name = now.format("%Y%m%d_%H%M%S").to_string();
    println!("{}", new_dir_name);

    let new_backup_directory = cfg.backup_dest.join(new_dir_name);
    println!("{:?}", new_backup_directory);
    fs::create_dir(new_backup_directory.as_path()).unwrap(); // TODO: Remove unwrap

    new_backup_directory
}

fn recursively_iterate_files(source_directoy: &Path, backup_directory: &Path) {
    for res in fs::read_dir(source_directoy).unwrap() { // TODO: Remove unwrap
        let dir_entry = res.unwrap(); // TODO: Remove unwrap
        let path = dir_entry.path();

        if path.is_file() {
            match backup_file(&path, backup_directory) {
                Ok(a) => a,
                Err(e) => println!("WARNING: Could not backup file {:?}: {:?}", path, e)
            };
        } else if path.is_dir() {
            recursively_iterate_files(&path, backup_directory);
        } else {
            println!("WARNING: Encountered neither file nor dir: {:?}", path);
        }
    }
}

fn backup_file(path: &Path, backup_directory: &Path) -> std::io::Result<()> {
    let new_path = backup_directory.join(path.clone());
    let new_dir = new_path.parent().unwrap_or(Path::new("/")); // TODO: Is that really the solution?
    // Creating new directory structure
    fs::create_dir_all(new_dir)?;
    // Copying file to new directory structure
    fs::copy(path, new_path)?;
    Ok(())
}

fn testing() {
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