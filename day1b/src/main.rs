use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use num_format::{Locale, ToFormattedString};

fn main() {
    // Number format locale.
    let locale = Locale::en;

    // Create a path to the desired file
    let path = Path::new("input.txt");
    let display = path.display();

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
    let mut highest: [i32; 3] = [0, 0, 0];
    for line in s.split('\n') {
        if line == "" {
            // println!(
            //     "Finished elf with total {}",
            //     total.to_formatted_string(&locale)
            // );
            if total > highest[0] {
                println!(
                    "Elf with {} beats lowest highest with {}",
                    total.to_formatted_string(&locale),
                    highest[0].to_formatted_string(&locale)
                );
                // Swap out the lowest for this total.
                highest[0] = total;
                println!("After repalcement, elves are now {:?}", highest);

                // Swap items in the array to keep it sorted.
                for i in 1..3 {
                    if highest[i] < highest[i - 1] {
                        let tmp = highest[i - 1];
                        highest[i - 1] = highest[i];
                        highest[i] = tmp;
                    } else {
                        break;
                    }
                }
                println!("After swap sorting, elves are now {:?}", highest);
            }
            total = 0;
        } else {
            total += line.parse::<i32>().unwrap();
        }
    }
    println!("{}", highest.iter().sum::<i32>());
}
