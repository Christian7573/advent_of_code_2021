use std::fs::File;
use std::io::Read;

fn read_dat(file: &str) -> String {
    let mut file = File::open(file).expect("Failed to open file");
    let mut dat = String::new();
    file.read_to_string(&mut dat).expect("Failed to read file");
    dat
}

fn main() {
    let dat = read_dat("dat.txt");    
    let mut prev = std::i64::MAX;
    let mut count = 0;
    for num in dat.trim().lines().map(|line| line.parse::<i64>().expect("NON NUMBER")) {
        if num > prev {
            count += 1;
        }
        prev = num;
    }
    println!("{}", count);

    let nums = dat.trim().lines().map(|line| line.parse::<i64>().expect("NON NUMBER")).collect::<Vec<_>>();
    let mut prev = std::i64::MAX;
    let mut count = 0;
    for i in 0..nums.len() {
        let sum = nums.get(i).unwrap_or(&0) + nums.get(i+1).unwrap_or(&0) + nums.get(i+2).unwrap_or(&0);        
        if sum > prev { count += 1; }
        prev = sum;
    }
    println!("{}", count);
}
