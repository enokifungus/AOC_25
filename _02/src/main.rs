use std::error::Error;
use regex;
use itertools::Itertools;

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
        let re = regex::Regex::new(r"[-,]")?;
        let a:Vec<_> = re.split(DATA1).map(|i| i.parse::<i32>()).collect::<Result<_,_>>()?;
        let a:Vec<(_,_)> = a.into_iter().tuples().collect();
        println!("{:?}",&a);
        for (i,j) in a {
            if i.ilog10() != j.ilog10() { println!("{i} {j}"); }
        }
        Ok(())
    }
}
