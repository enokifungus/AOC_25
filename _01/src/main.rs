use std::fs::read_to_string;

fn part1(sc: &Vec<i32>) -> i32 {
    let mut x = 50;
    let mut n = 0;
    for i in sc {
        x = (x + i).rem_euclid(100);
        if x==0 { n += 1; }
    }
    n
}

fn parse_source(sc: &str) -> Vec<i32> {
    sc.lines()
        .map(|i| i.split_at(1))
        .map(|(d,i)| i.parse::<i32>().unwrap() * (if d=="L" {-1} else {1}))
        .collect()
}

fn main() {
    let sc = read_to_string("data").unwrap();
    let sc = parse_source(&sc);
    let n = part1(&sc);
    println!("PART1: {}",n);
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
        let sc = parse_source(DATA1);
        let n = part1(&sc);
        assert_eq!(n, 3);
    }
}
