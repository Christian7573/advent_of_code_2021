use std::fs::File;
use std::io::Read;
use std::collections::BTreeMap;

fn read_dat(file: &str) -> String {
    let mut file = File::open(file).expect("Failed to open file");
    let mut dat = String::new();
    file.read_to_string(&mut dat).expect("Failed to read file");
    dat
}

fn main() {
    let dat = read_dat("dat.txt");    
    let mut field: BTreeMap<(i64, i64), i64> = BTreeMap::new();

    for line in dat.trim().lines() {
        let mut bits1 = line.split(" -> ");
        let mut bits2 = bits1.next().unwrap().split(",");
        let mut bits3 = bits1.next().unwrap().split(",");
        let x1: i64 = bits2.next().unwrap().parse().unwrap();
        let y1: i64 = bits2.next().unwrap().parse().unwrap();
        let x2: i64 = bits3.next().unwrap().parse().unwrap();
        let y2: i64 = bits3.next().unwrap().parse().unwrap();

        if x1 == x2 {
            let my1 = y1.min(y2);
            let my2 = y1.max(y2);
            for i in my1..=my2 {
                let point = (x1, i);
                let mut yes = field.get(&point).map(|x| *x).unwrap_or(0);
                yes += 1;
                field.insert((x1, i), yes);
            }
        } else if y1 == y2 {
            let my1 = x1.min(x2);
            let my2 = x1.max(x2);
            for i in my1..=my2 {
                let point = (i, y1);
                let mut yes = field.get(&point).map(|x| *x).unwrap_or(0);
                yes += 1;
                field.insert(point, yes);
            }
        //THIS ONE IS FOR PART 2
        } else {
            let xic = if x1 < x2 { 1 } else { -1 };
            let yic = if y1 < y2 { 1 } else { -1 };            
            let mut x = x1;
            let mut y = y1;
            let point = (x, y);
            let mut yes = field.get(&point).map(|x| *x).unwrap_or(0);
            yes += 1;
            field.insert(point, yes);
            while x != x2 && y != y2 {
                if x != x2 { x += xic };
                if y != y2 { y += yic };
                let point = (x, y);
                let mut yes = field.get(&point).map(|x| *x).unwrap_or(0);
                yes += 1;
                field.insert(point, yes);
            }
        }
        //END PART 2 CODE
    }

    let mut count = 0;
    for ((x, y), num) in field {
        if num > 1 { 
            println!("{} {} {}", x, y, num);
            count += 1;
        }        
    }
    println!("{}", count);

}
