use std::error::Error;
use std::fs::read_to_string;
use std::collections::HashSet;

fn part12(a: &Vec<(i64,i64)>) -> (i64,i64) {
    let mut b = HashSet::new();
    let mut c = HashSet::new();
    for &(p,q) in a {
        let n = p.ilog10()+1;
        let r = |x| {
            let d = (10i64.pow(n) - 1)/(10i64.pow(x) - 1);
            let mut s = p/10i64.pow(n - x)*d;
            if s < p { s += d; }
            (s..q).step_by(d as usize)
        };
        if n%2 == 0 {
            c.extend(r(n/2));
        }
        for x in 1..n/2+1 {
            if n%x == 0 {
                b.extend(r(x));
            }
        }
    }
    (c.into_iter().sum::<i64>(),b.into_iter().sum::<i64>())
}

fn parse_source(sc: &str) -> Result<Vec<(i64,i64)>,Box<dyn Error>> {
    let mut z = Vec::new();
    for i in sc.split(',') {
        let (p,q) = i.split_once('-').ok_or("expected 'number-number'")?;
        let (mut p,q) = (p.parse::<i64>()?,q.parse::<i64>()?);
        for j in p.ilog10()+1..=q.ilog10() {
            let r = 10i64.pow(j);
            z.push((p,r));
            p = r;
        }
        z.push((p,q+1));
    }
    Ok(z)
}

fn main() -> Result<(),Box<dyn Error>> {
    let sc = read_to_string("data")?;
    let sc = sc.trim();
    let a = parse_source(&sc)?;
    let (n1,n2) = part12(&a);
    println!("PART1: {n1}");
    println!("PART2: {n2}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA1:&str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn _12() -> Result<(),Box<dyn Error>> {
        let a = parse_source(DATA1)?;
        let (n1,n2) = part12(&a);
        assert_eq!(n1,1227775554);
        assert_eq!(n2,4174379265);
        Ok(())
    }
}
