use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp::max;



fn main() {

    let mut maxi = 0;
    let mut accu = 0; 
    let mut temp = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);


                match ip.parse::<i32>() {
                    Ok(num) => temp += num,
                    Err(_) => temp = 0,
                }
                
                maxi = max(maxi,temp);

                println!("{}", maxi);
            }
        }
    }
}



fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}



