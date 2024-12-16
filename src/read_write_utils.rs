use std::{
    fs::{File, create_dir},
    path::Path,
    io::{BufRead, BufReader, Lines, Write},
};

pub fn write_file(path: &Path, temp_file: &Path) {
    let dir = Path::new("./output");
    if !dir.is_dir() {
        let c_dir = create_dir(dir);
        match c_dir {
            Ok(s) => println!("output directory created: {:?}", s),
            Err(e) => eprintln!("Failed to create output directory: {}", e),
        }
    }

    let full_path = format!("{}/{}.csv", dir.to_string_lossy(), path.to_string_lossy());
    println!("{:?}", full_path);
    let mut file = match File::create(full_path) {
        Ok(s) => s,
        Err(e) => panic!("Failed to write file: {}", e),
    };

    let data = read_temp(temp_file);
    
    for record in data {
        let written = file.write(format!("{}\n", record).as_bytes());
        match written {
            Ok(s) => println!("Write succesful: {}", s),
            Err(e) => eprintln!("Failed to write line: {}", e),
        }
    }
}

pub fn read_temp(path: &Path) -> Vec<String> {
    let file = match File::open(&path) {
        Ok(s) => s,
        Err(e) => panic!("Failed to read temp file: {}", e),
    };
    let buf = BufReader::new(file);
    let mut record: Vec<String> = Vec::new();
    //let new_iter = get_temp_iter(buf);
    
    for line in buf.lines() {
        record.push(line.unwrap());
    }

    return record
}

fn get_temp_iter(buffer: BufReader<File>, old_iter: Lines<BufReader<File>>) -> Lines<BufReader<File>> {
    let iter = buffer.lines();

    return iter
}
