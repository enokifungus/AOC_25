use std::error::Error;
use regex;
use itertools::Itertools;

fn parse_source(sc: &str) -> Result<Vec<(i32,i32)>,Box<dyn Error>> {
    let mut z = Vec::new();
    for i in sc.split(',') {
        let a:Vec<_> = i.split('-').map(|j| j.parse::<i32>()).collect::<Result<_,_>>()?;
        let [mut x,y] = a[..].try_into()?;
        for e in x.ilog10()..y.ilog10() {
            let a = 
        }
        z.push((x,y));
    }
    Ok(z)
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    //const DATA1:&str = "1-2-8-4";
    const DATA1:&str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn _1() -> Result<(),Box<dyn Error>> {
        let a = parse_source(DATA1);
        println!("{:?}",&a);
        Ok(())
    }
}
