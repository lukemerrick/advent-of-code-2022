use std::cmp;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // Create a path to the desired file
    let path = Path::new("input.txt");
    let display = path.display();
    // println!("Reading {}...", display);

    // Read the file.
    let mut s = String::new();
    {
        
        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        // Read the file contents into a string, returns `io::Result<usize>`
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => (),
        }

        // `file` goes out of scope, and the "hello.txt" file gets closed
    }

    // Go line by line.
    let mut total: i32 = 0;
    let mut highest: i32 = 0;
    for line in s.split('\n') {
        if line == "" {
            highest = cmp::max(highest, total);
            total = 0;
        }
        else {
            total += line.parse::<i32>().unwrap();
        }
    }
    println!("{}", highest);
}