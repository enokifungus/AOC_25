use std::fs::read_to_string;

fn strip2(s: &str,pr: char,su: char) -> &str {
    s.strip_prefix(pr).unwrap().strip_suffix(su).unwrap()
}

#[derive(Debug)]
struct Machine {
    st: Vec<u32>,
    tg: Vec<u16>,
    tn: u16,
    jt: Vec<u32>,
}

impl Machine {
    fn new(sc: &str) -> Self {
        let mut a:Vec<_> = sc.split(' ').collect();
        let st:Vec<_> = strip2(a.remove(0),'[',']').chars().map(|c| (c != '.') as u32).collect();
        let jt:Vec<_> = strip2(a.pop().unwrap(),'{','}').split(',').map(|t| t.parse().unwrap()).collect();
        let mut tg = vec![0u16;st.len()];
        let tn = 1<<a.len();
        let mut b = 1;
        for s in a {
            for i in strip2(s,'(',')').split(',').map(|t| t.parse::<usize>().unwrap()) {
                tg[i] |= b;
            }
            b += b;
        }
        Machine { st,tg,tn,jt }
    }

    fn fits_start(&self, i: u16) -> bool {
        (0..self.st.len()).all(|j| (self.tg[j]&i).count_ones()&1 == self.st[j])
    }

    fn fit_joltage(&self, jt: &[u32], p2:u32) -> Option<u32> {
        if jt.iter().all(|&i| i == 0) { return Some(0); }
        let mut n = u32::MAX;
        for i in 0..self.tn {
            if let Some(a) = (0..jt.len()).map(|j| { let x = (i&self.tg[j]).count_ones(); if x&1 == jt[j]&1 { jt[j].checked_sub(x).map(|x| x/2) } else { None } }).collect::<Option<Vec<_>>>() {
                if let Some(b) = self.fit_joltage(&a,p2+p2) {
                    n = n.min(i.count_ones()*p2 + b);
                }
            }
        }
        (n<u32::MAX).then_some(n)
    }
}

fn part1(ar: &[Machine]) -> u32 {
    ar.iter().map(|m| (0..m.tn).filter(|&i| m.fits_start(i))
        .map(|i| i.count_ones()).min().unwrap()).sum::<u32>()
}

fn part2(ar: &[Machine]) -> u32 {
    ar.into_iter().map(|m| m.fit_joltage(&m.jt,1).expect("joltage not solved")).sum::<u32>()
}

fn main() {
    let sc = read_to_string("data").expect("file 'data'");
    let ar:Vec<_> = sc.lines().map(|s| Machine::new(s)).collect();
    let n1 = part1(&ar);
    let n2 = part2(&ar);
    println!("PART1: {n1}");
    println!("PART2: {n2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA:&str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn _1() {
        let sc = DATA;
        let ar:Vec<_> = sc.lines().map(Machine::new).collect();
        let n1 = part1(&ar);
        let n2 = part2(&ar);
        assert_eq!(n1,7);
        assert_eq!(n2,33);
    }
}
