use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn main() -> std::io::Result<()> {
    println!("aoc_01a");
    aoc_01a()?;
    println!("aoc_01b");
    aoc_01b()?;
    Ok(())
}

fn aoc_01a() -> std::io::Result<()> {
    let f = File::open("aoc_01a.txt")?;
    let reader = BufReader::new(f);

    let mut nums = reader.lines().map(|l| l.unwrap().parse::<u32>());
    let mut check = nums.next().unwrap().unwrap();
    let mut incs:u32 = 0;
    for num in nums {
        match num {
            Ok(n) => {
                if check < n {
                    incs = incs + 1;
                }
                check = n
            }
            Err(_) =>
                println!("Something bad"),
        }
        
    }
    println!("{}", incs);
    Ok(())
}

fn aoc_01b() -> std::io::Result<()> {
    let f = File::open("aoc_01b.txt")?;
    let reader = BufReader::new(f);

    let mut nums = reader.lines().map(|l| l.unwrap().parse::<u32>());

    let a = nums.next().unwrap().unwrap();
    let mut b = nums.next().unwrap().unwrap();
    let mut c = nums.next().unwrap().unwrap();
    let mut check = a + b + c;
    let mut incs:u32 = 0;
    for num in nums {
        match num {
            Ok(d) => {
                if check < b + c + d {
                    incs = incs + 1;
                }
                check = b + c + d;
                b = c;
                c = d
            }
            Err(_) =>
                println!("Something bad"),
        }
        
    }
    println!("{}", incs);
    Ok(())
}
