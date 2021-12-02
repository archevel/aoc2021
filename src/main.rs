use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn main() -> std::io::Result<()> {
    println!("aoc_01a");
    aoc_01a()?;
    println!("aoc_01b");
    aoc_01b()?;
    println!("aoc_02a");
    aoc_02a()?;
    println!("aoc_02b");
    aoc_02b()?;
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

fn aoc_02a() -> std::io::Result<()> {
    let f = File::open("aoc_02a.txt")?;
    let reader = BufReader::new(f);

    let lines = reader.lines().map(|l| l.unwrap());

    let mut depth = 0;
    let mut horiz = 0;
    for line in lines {
        let mut split = line.split_whitespace();
        let cmd = split.next();
        let magnitude:u32 = split.last().unwrap().parse().unwrap();
        match cmd {
            Some("forward") => {
                horiz = horiz + magnitude;
            },
            Some("up") => {
                depth = depth - magnitude;
            },
            Some("down") =>  {
                depth = depth + magnitude;
            },
            _ => println!("Something bad..."),
        }
        
    }
    println!("{}", depth * horiz);
    Ok(())
}

fn aoc_02b() -> std::io::Result<()> {
    let f = File::open("aoc_02b.txt")?;
    let reader = BufReader::new(f);

    let lines = reader.lines().map(|l| l.unwrap());

    let mut aim = 0;
    let mut horiz = 0;
    let mut depth = 0;
    
    for line in lines {
        let mut split = line.split_whitespace();
        let cmd = split.next();
        let magnitude:u32 = split.last().unwrap().parse().unwrap();
        match cmd {
            Some("forward") => {
                horiz = horiz + magnitude;
                depth = depth + aim * magnitude;
            },
            Some("up") => {
                aim = aim - magnitude;
            },
            Some("down") =>  {
                aim = aim + magnitude;
            },
            _ => println!("Something bad..."),
        }
        
    }
    println!("{}", depth * horiz);
    Ok(())
}