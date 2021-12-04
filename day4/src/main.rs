use std::fs::File;
use std::io::Read;

fn read_dat(file: &str) -> String {
    let mut file = File::open(file).expect("Failed to open file");
    let mut dat = String::new();
    file.read_to_string(&mut dat).expect("Failed to read file");
    dat
}

#[derive(Clone)]
struct Bingo {
    numbers: [i64; 25],
    gots: [bool; 25],
}

impl Bingo {
    fn advance(&mut self, num: i64) -> bool {
        for i in 0..self.numbers.len() {
            if (self.numbers[i] == num) { self.gots[i] = true; }
        }
        for i in 0..5 { if self.check_vertical(i) { return true }}
        for i in 0..5 { if self.check_horizontal(i) { return true }}
        self.check_diagonal()
    }

    fn check_vertical(&self, offset: usize) -> bool {
            self.gots[0 + offset] &&
            self.gots[5 + offset] &&
            self.gots[10 + offset] &&
            self.gots[15 + offset] &&
            self.gots[20 + offset]
    }
    fn check_horizontal(&self, offset: usize) -> bool {
        self.gots[offset * 5 + 0] &&
        self.gots[offset * 5 + 1] &&
        self.gots[offset * 5 + 2] &&
        self.gots[offset * 5 + 3] &&
        self.gots[offset * 5 + 4]
    }

    fn check_diagonal(&self) -> bool {
        (
            self.gots[0] &&
            self.gots[6] &&
            self.gots[12] &&
            self.gots[18] &&
            self.gots[24]
        ) || (
            self.gots[4] &&
            self.gots[8] &&
            self.gots[12] &&
            self.gots[16] &&
            self.gots[20]
        )
    }

    fn prescore(&self) -> i64 {
        let mut aggregate = 0;
        for i in 0..self.numbers.len() {
            if !self.gots[i] { aggregate += self.numbers[i] }
        }
        aggregate
    }
}

fn main() {
    let dat = read_dat("dat.txt");    
    let mut lines = dat.trim().lines();

    let callout = lines.next().unwrap();
    lines.next().unwrap();

    let mut bingos_my = Vec::new();
    while let Some(line1) = lines.next() {
        let mut nums = [0; 25];
        let mut i = 0;
        let mut line = line1;
        while i < 25 {
            for num in line.split(" ") {
                if let Ok(num) = num.parse::<i64>() {
                    nums[i] = num;
                    i += 1;
                } else {
                    eprintln!("REE \"{}\"", line);
                }
            }
            line = lines.next().unwrap_or("");
        }
        bingos_my.push(Bingo { numbers: nums, gots: [false; 25] });
    }

    let mut bingos = bingos_my.clone();
    for num in callout.split(",") {
        let num = num.parse::<i64>().unwrap();
        for bingo in &mut bingos {
            if bingo.advance(num) {
                println!("{}", bingo.prescore() * num);
            }
        }
    }
    println!("=====");
    let mut bingos = bingos_my.clone();
    for num in callout.split(",") {
        let num = num.parse::<i64>().unwrap();
        let mut i = 0;
        while i < bingos.len() {
            if bingos.get_mut(i).unwrap().advance(num) {
                if bingos.len() == 1 {
                    println!("{}", bingos[0].prescore() * num);
                }
                bingos.remove(i);
            } else {
                i += 1;
            }
        }
    }
}
