use itertools::Itertools;
use std::fs::read;

fn part12(da: &[u8]) -> (i64,i64) {
    let wd = da.iter().position(|&x| x == b'\n').unwrap();
    let rw = da.len() - wd;
    let mut op:Vec<_> = (0..wd).filter(|i| da[rw + i] != b' ').collect();
    op.push(wd + 1);
    let ir = (0..rw).step_by(wd + 1);
    let mut n1 = 0;
    let mut n2 = 0;
    for (&i,&j) in op.iter().tuple_windows() {
        let ic = i..j-1;
        let it = ir.clone()
            .map(|k| ic.clone().map(|l| da[k + l] as char).collect::<String>().trim().parse::<i64>().unwrap());
        let x = if da[rw + i] == b'+' { it.sum::<i64>() } else { it.product::<i64>() };
        n1 += x;
        let it = ic.clone()
            .map(|k| ir.clone().map(|l| da[k + l] as char).collect::<String>().trim().parse::<i64>().unwrap());
        let x = if da[rw + i] == b'+' { it.sum::<i64>() } else { it.product::<i64>() };
        n2 += x;
    }
    (n1,n2)
}

fn main() {
    let da = read("data").expect("file 'data'");
    let (n1,n2) = part12(&da);
    println!("PART1: {n1}");
    println!("PART2: {n2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA:&[u8] = b"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn _1() {
        let (n1,n2) = part12(DATA);
        assert_eq!(n1,4277556);
        assert_eq!(n2,3263827);
    }
}
