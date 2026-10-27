use std::fs::read_to_string;

#[derive(Debug,Clone)]
struct Board {
    bd: Vec<Vec<u8>>,
    wd: i32,
    ht: i32
}

impl Board {
    fn new(a:&str) -> Self {
        let bd:Vec<Vec<_>> = a.lines()
            .map(|s| s.chars().map(|c| (c == '@') as u8).collect())
            .collect();
        let wd = bd[0].len() as i32;
        let ht = bd.len() as i32;
        Board { bd,wd,ht }
    }

    fn solve(&mut self) -> (i32,i32) {
        let mut a = self.bd.clone();
        let mut n1 = None;
        let mut n2 = 0;
        loop {
            let mut n = 0;
            for r in 0..self.ht {
                for c in 0..self.wd {
                    if self.bd[r as usize][c as usize] == 1 && self.neighbors(r,c) < 4 {
                        a[r as usize][c as usize] = 0;
                        n += 1;
                    }
                }
            }
            if n1.is_none() { n1 = Some(n); }
            n2 += n;
            if n == 0 { break; }
            self.bd.clear();
            self.bd.extend(a.iter().cloned());
        }
        (n1.expect("error value expected"),n2)
    }

    fn neighbors(&self, r:i32, c:i32) -> i32 {
        let mut n = 0;
        if let Some(a) = (r > 0).then(|| &self.bd[(r - 1) as usize]) {
            if c > 0 { n += a[(c - 1) as usize]; }
            n += a[c as usize];
            if c < self.wd - 1 { n += a[(c + 1) as usize]; }
        }
        let a = &self.bd[r as usize];
        if c > 0 { n += a[(c - 1) as usize]; }
        //n += a[c as usize];
        if c < self.wd - 1 { n += a[(c + 1) as usize]; }
        if let Some(a) = (r < self.ht - 1).then(|| &self.bd[(r + 1) as usize]) {
            if c > 0 { n += a[(c - 1) as usize]; }
            n += a[c as usize];
            if c < self.wd - 1 { n += a[(c + 1) as usize]; }
        }
        n as i32
    }
}

fn main() {
    let sc:String = read_to_string("data").expect("file not found");
    let mut sc = Board::new(&sc);
    let (n1,n2) = sc.solve();
    println!("PART1: {n1}");
    println!("PART2: {n2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA1:&str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn _1() {
        let mut sc = Board::new(DATA1);
        let (n1,n2) = sc.solve();
        assert_eq!(n1,13);
        assert_eq!(n2,43);
    }
}
