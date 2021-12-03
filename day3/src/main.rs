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
    
    let mut a = String::new();
    let mut b = String::new();
    let len = dat.trim().lines().next().unwrap().len();
    for i in 0..len {
        let mut num1s = 0;
        let mut num0s = 0;
        for line in dat.trim().lines() {
            let charr = line.as_bytes()[i] as char;
            if charr == '1' { num1s += 1; }
            else if charr == '0' { num0s += 1; }
            else { panic!() }
        }
        if num1s > num0s { a += "1" }
        else { a += "0" }
        if num1s < num0s { b += "1" }
        else { b += "0" }
    }
    let a = i64::from_str_radix(&a, 2).unwrap();
    let b = i64::from_str_radix(&b, 2).unwrap();
    println!("{}", a * b);

    let mut lines = dat.trim().lines().collect::<Vec<_>>();
    let mut i = 0;
    while lines.len() > 1 {
        let mut num1s = 0;
        let mut num0s = 0;
        for line in &lines {
            let charr = line.as_bytes()[i] as char;
            if charr == '1' { num1s += 1; }
            else if charr == '0' { num0s += 1; }
            else { panic!() }
        }
        let bit = if num1s > num0s { '1' } else if num0s > num1s { '0' } else { '1' };
        lines = lines.into_iter().filter(|x| x.as_bytes()[i] as char == bit).collect();        
        i += 1;
    }
    let a = i64::from_str_radix(&lines[0], 2).unwrap();
    println!("{}", a);
    let mut lines = dat.trim().lines().collect::<Vec<_>>();
    let mut i = 0;
    while lines.len() > 1 {
        let mut num1s = 0;
        let mut num0s = 0;
        for line in &lines {
            let charr = line.as_bytes()[i] as char;
            if charr == '1' { num1s += 1; }
            else if charr == '0' { num0s += 1; }
            else { panic!() }
        }
        let bit = if num1s < num0s { '1' } else if num0s < num1s { '0' } else { '0' };
        lines = lines.into_iter().filter(|x| x.as_bytes()[i] as char == bit).collect();        
        i += 1;
    }
    let b = i64::from_str_radix(&lines[0], 2).unwrap();
    println!("{}", b);
    println!("{}", a * b);
}
