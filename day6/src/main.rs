use std::fs::File;
use std::io::Read;
use std::collections::BTreeMap;

fn read_dat(file: &str) -> String {
    let mut file = File::open(file).expect("Failed to open file");
    let mut dat = String::new();
    file.read_to_string(&mut dat).expect("Failed to read file");
    dat = dat.trim().to_string();
    dat
}

fn main() {
    let dat = read_dat("dat.txt");    
    let mut fishi = [0u64, 0, 0, 0, 0, 0, 0, 0, 0];
    for fish in dat.split(",") {
        if let Ok(fish) = fish.parse::<usize>() {
            fishi[fish] += 1;            
        } else {
            panic!("`{}`", fish);
        }
    }
    //let mut fishi = [0, 1, 1, 2, 1, 0, 0, 0, 0];
    println!("{:?}", fishi);

    for _i in 0..256 {
        let f = fishi;
        fishi = [f[1], f[2], f[3], f[4], f[5], f[6], f[7] + f[0], f[8], f[0]];
    }

    let mut sum = 0;
    for i in 0..fishi.len() { sum += fishi[i] }
    println!("{} {:?}", sum, fishi);
}
