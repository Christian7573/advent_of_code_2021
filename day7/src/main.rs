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
    let crabs = dat.split(",").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
    let mut num_pos = BTreeMap::new();
    for pos in &crabs {
        let mut count = *num_pos.get(pos).unwrap_or(&0);
        count += 1;
        num_pos.insert(*pos, count);
    }
    let min = *num_pos.keys().min().unwrap();
    let max = *num_pos.keys().max().unwrap();
    let least_fuel = (min..=max).map(|goto| crabs.iter().map(|pos| (pos - goto).abs()).sum::<i64>()).min();
    println!("{:?}", least_fuel);

    let least_fuel = (min..=max).map(|goto| crabs.iter().map(|pos| {
        let dist = (pos - goto).abs();
        if dist == 0 { 0 }
        else {
            dist * (dist + 1) / 2
        }
    }).sum::<i64>()).min();
    println!("{:?}", least_fuel);
}
