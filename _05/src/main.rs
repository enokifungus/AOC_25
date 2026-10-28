use std::fs::read_to_string;

fn part12(fr: Vec<(i64,i64)>, ig: Vec<i64>) -> (usize,i64) {
    let n1 = ig.into_iter().filter(|i| fr.iter().any(|(a,b)| a <= i && i <= b))
        .count();
    let mut u = Vec::new();
    for &(mut a,mut b) in fr.iter() {
        let mut v = Vec::new();
        for i in 0..u.len() {
            let (c,d) = u[i];
            if a <= d+1 && b >= c-1 {
                a = a.min(c);
                b = b.max(d);
                v.insert(0,i);
            }
        }
        for i in v { u.remove(i); }
        u.push((a,b));
    }
    let n2 = u.into_iter().map(|(a,b)| b - a + 1).sum::<i64>();
    (n1,n2)
}

fn parse_source(sc: &str) ->(Vec<(i64,i64)>,Vec<i64>) {
    let mut it = sc.lines();
    let tonum = |s:&str| s.parse::<i64>().expect("invalid number");
    let a = (0..).map_while(|_| it.next().and_then(|i| i.split_once('-')))
        .map(|(a,b)| (tonum(a),tonum(b)))
        .collect();
    let b = it.map(|i| tonum(i)).collect();
    (a,b)
}

fn main() {
    let sc = read_to_string("data").expect("file 'data'");
    let (fr,ig) = parse_source(&sc);
    let (n1,n2) = part12(fr,ig);
    println!("PART1: {n1}");
    println!("PART2: {n2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA:&str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn _1() {
        let (fr,ig) = parse_source(DATA);
        let (n1,n2) = part12(fr,ig);
        assert_eq!(n1,3);
        assert_eq!(n2,14);
    }
}
