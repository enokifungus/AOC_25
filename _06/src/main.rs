use itertools::Itertools;

fn parse_source(sc: &str) -> Vec<(Vec<Vec<u8>>,u8)> {
    let mut sc:Vec<_> = sc.lines().map(|i| i.as_bytes()).collect();
    let s = sc.pop().expect("operator line");
    let mut op:Vec<_> = s.iter().enumerate()
        .filter_map(|(i,&c)| (c != b' ').then(|| (i,c))).collect();
    op.push((s.len()+1,0));
    op.iter().tuple_windows()
        .map(|(&(i,o),&(j,_))|
            (sc.iter().map(|s| s[i..j-1].to_vec()).collect::<Vec<_>>(),o))
        .collect::<Vec<_>>()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA:&str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn _1() {
        let da = parse_source(DATA);
        let n1 = da.iter().map(|(a,o)| {
            let b:Vec<_> = a.iter().map(|u| String::from_utf8(u.clone()).unwrap().trim().parse::<i32>().unwrap()).collect();
            if *o == b'+' { b.into_iter().sum::<i32>() }
            else { b.into_iter().product::<i32>() }
        }).sum::<i32>();
        println!("{n1:?}");
    }
}
