use std::fs::read_to_string;
use std::error::Error;

fn str2jolt(n: usize, s: &str) -> Result<i64,Box<dyn Error>> {
    let mut a = vec![0u8; n];
    let mut si = s.chars().map(|c| c as u8);
    for i in 0..n { a[i] = si.next().ok_or("line is too short")?; }
    while let Some(c) = si.next() {
        if let Some(i) = (0..n-1).find(|&i| a[i] < a[i+1]) {
            a.copy_within(i+1..,i);
            a[n-1] = c;
        } else if c > a[n-1] {
            a[n-1] = c;
        }
    }
    let x = std::str::from_utf8(&a)?;
    x.parse().map_err(Into::into)
}

fn main() -> Result<(),Box<dyn Error>> {
    let sc = read_to_string("data").unwrap();
    let n = sc.lines().map(|i| str2jolt(2,i)).sum::<Result<i64,_>>()?;
    println!("PART1: {n}");
    let n = sc.lines().map(|i| str2jolt(12,i)).sum::<Result<i64,_>>()?;
    println!("PART2: {n}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA1:&str = "987654321111111
811111111111119
234234234234278
818181911112111";
    
    #[test]
    fn _12() {
        let sc = DATA1;
        let n1 = sc.lines().map(|i| str2jolt(2,i)).sum::<Result<i64,_>>().unwrap();
        let n2 = sc.lines().map(|i| str2jolt(12,i)).sum::<Result<i64,_>>().unwrap();
        assert_eq!(n1,357);
        assert_eq!(n2,3121910778619);
    }
}
