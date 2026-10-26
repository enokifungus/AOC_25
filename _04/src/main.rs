use std::fs::read_to_string;

#[derive(Debug)]
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

    fn neighbors(&self, r:i32, c:i32) -> i32 {
        let mut n = 0;
        if let Some(a) = (r > 0).then(|| &self.bd[(r - 1) as usize]) {
            if c > 0 { n += a[(c - 1) as usize]; }
            n += a[c as usize];
            if c < self.wd - 1 { n += a[(c + 1) as usize]; }
        }
        let a = &self.bd[r as usize];
        if c > 0 { n += a[(c - 1) as usize]; }
        n += a[c as usize];
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
    let sc:String = read_to_string("data").unwrap();
    let sc = Board::new(&sc);
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
        let sc = Board::new(DATA1);
        println!("{sc:?}");
        let n = (0..sc.ht).flat_map(|i| (0..sc.wd).map(move |j| (i,j))).map(|(r,c)| (sc.bd[r as usize][c as usize] == 1 && sc.neighbors(r,c) < 4) as i32).sum::<i32>();
        println!("{n}");
    }
}
