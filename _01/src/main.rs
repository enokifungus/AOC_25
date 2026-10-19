use std::error::Error;
use std::num::ParseIntError;
use std::fs::read_to_string;

fn part12(sc: &Vec<i32>) -> (i32,i32) {
    let mut x = 50;
    let mut n1 = 0;
    let mut n2 = 0;
    for &i in sc {
        if i > 0 {
            n2 += (x + i)/100;
        } else if x > 0 {
            n2 += (100 - x - i)/100;
        } else {
            n2 += (-i)/100;
        }
        x = (x + i).rem_euclid(100);
        if x==0 { n1 += 1; }
    }
    (n1,n2)
}

fn parse_source(sc: &str) -> Result<Vec<i32>,ParseIntError> {
    sc.lines()
        .map(|i| i.split_at(1))
        .map(|(d,i)| i.parse::<i32>().map(|x| x * (if d=="L" {-1} else {1})))
        .collect()
}

fn main() -> Result<(),Box<dyn Error>> {
    let sc = read_to_string("data")?;
    let sc = parse_source(&sc)?;
    let (n1,n2) = part12(&sc);
    println!("PART1: {}",n1);
    println!("PART2: {}",n2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA1:&str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn _1() {
        let sc = parse_source(DATA1).unwrap();
        let (n,_) = part12(&sc);
        assert_eq!(n, 3);
    }

    #[test]
    fn _2() {
        let sc = parse_source(DATA1).unwrap();
        let (_,n) = part12(&sc);
        assert_eq!(n, 6);
    }
}
