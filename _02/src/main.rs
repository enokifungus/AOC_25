use std::error::Error;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA1:&str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn _1() -> Result<(),Box<dyn Error>> {
        let a:Vec<_> = DATA1.split(',').map(|i| i.split('-').map(|j| j.parse::<i32>().collect::<Result<_,_>>()).collect::<Result<_,_>>()?;
        println!("{:?}",a);
        Ok(())
    }
}
