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

    let mut forward: i64 = 0;
    let mut depth: i64 = 0;
    for line in dat.trim().lines() {
        let line = line.trim();
        if line.len() > 8 && &line[0..8] == "forward " {
            forward += line[8..].parse::<i64>().expect("Non number");
        } else if line.len() > 5 && &line[0..5] == "down " {
            depth += line[5..].parse::<i64>().expect("Non number");
        } else if line.len() > 3 && &line[0..3] == "up " {
            depth -= line[3..].parse::<i64>().expect("Non number");
        } else {
            panic!("Invalid: {}", line);
        }
    }
    println!("{} {} {}", forward, depth, forward * depth);

    let mut forward: i64 = 0;
    let mut aim: i64 = 0;
    let mut depth: i64 = 0;
    for line in dat.trim().lines() {
        let line = line.trim();
        if line.len() > 8 && &line[0..8] == "forward " {
            let val = line[8..].parse::<i64>().expect("Non number");
            forward += val;
            depth += val * aim;
        } else if line.len() > 5 && &line[0..5] == "down " {
            aim += line[5..].parse::<i64>().expect("Non number");
        } else if line.len() > 3 && &line[0..3] == "up " {
            aim -= line[3..].parse::<i64>().expect("Non number");
        } else {
            panic!("Invalid: {}", line);
        }
    }
    println!("{} {} {}", forward, depth, forward * depth);
}
